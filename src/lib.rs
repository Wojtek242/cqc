//! # CQC Interface Library
//!
//! The Classical-Quantum Combiner (CQC) interface is used to program quantum
//! networking nodes to create, transmit, and manipulate qubits.
//!
//! The CQC interface will be used to interact with the Dutch demonstration
//! network, currently under development at QuTech in the Netherlands. At
//! present, the CQC interface is supported only by the quantum network
//! simulator [Simulaqron](http://www.simulaqron.org/).
//!
//! This crate provides an encoder and decoder for CQC protocol packets.  It
//! does not provide any I/O capabilities in order to maximise reusability by
//! not putting any runtime constraints on the user.
//!
//! ## Principle of Operation
//!
//! This library provides two functions:
//!
//! 1) Build valid CQC packets.
//!
//! 2) Encode/decode to/from binary format.  It is left to the user to decide
//! how best to fit I/O in their framework.
//!
//! ### Building Packets
//!
//! This crate offers two ways of building packets
//!
//! 1) Manually - one can manually build packets using the header definitions
//! and documentation provided in the `hdr` module.
//!
//! 2) Using the `builder` module - the builder module provides a simple API
//! for generating CQC packets.  It should be used in conjunction with the CQC
//! interface documentation in the `hdr` module.
//!
//! ### Encoding/decoding packets
//!
//! All headers in the `hdr` module implement `serde`'s `Serialize` and
//! `Deserialize` traits which mean they can be directly used as input to
//! `bincode`.  The `Encoder` and `Decoder` impls provide an example.
//!
//! The `builder` module returns a `Request` struct which implements
//! `Serialize` which can be used with `bincode`.
//!
//! The library provides a `Response` struct which implements `Deserialize` and
//! can be used to deserialize any response from the SimulaQron server.
//!
//! ### Example
//!
//! The following example will create a qubit on one node and send it to
//! another node.  Before running the example below start up the SimulaQron
//! nodes with `$NETSIM/run/startAll.sh --nrnodes 2`.
//!
//! ```no_run
//! extern crate bincode;
//! extern crate cqc;
//!
//! use cqc::builder;
//! use cqc::hdr;
//! use std::net;
//!
//! // Initialise local node `localhost:8803`.
//! let hostname = String::from("localhost");
//! let local_port: u16 = 8803;
//!
//! // Set up remote node `127.0.0.1:8804`.
//! let remote_host: u32 = u32::from(net::Ipv4Addr::new(127, 0, 0, 1));
//! let remote_port: u16 = 8804;
//!
//! // Initialise application state with ID 10.
//! let app_id: u16 = 10;
//! let client = builder::Client::new(app_id);
//! let mut coder = bincode::config();
//! coder.big_endian();
//!
//! // Create, and send a qubit from `localhost:8803` to `localhost:8804`.
//! {
//!     // Open connection to local node.
//!     let stream = net::TcpStream::connect((hostname.as_str(), local_port))
//!         .expect("Connect failed");
//!
//!     // Create the qubit.
//!     let request = client.cmd_new(0, hdr::CmdOpt::empty());
//!     coder.serialize_into(&stream, &request).expect(
//!         "Sending failed",
//!     );
//!
//!     // Wait for confirmation of creation.
//!     let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");
//!
//!     // Read the created qubit ID.
//!     let note = response.notify.get_qubit_hdr();
//!     let qubit_id = note.qubit_id;
//!
//!     // Send the qubit to the remote node.
//!     let request = client.cmd_send(
//!         qubit_id,
//!         *hdr::CmdOpt::empty().set_notify(),
//!         builder::RemoteId {
//!             remote_app_id: app_id,
//!             remote_node: remote_host,
//!             remote_port: remote_port,
//!         },
//!     );
//!     coder.serialize_into(&stream, &request).expect(
//!         "Sending failed",
//!     );
//!
//!     // Wait for confirmation.
//!     let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");
//!     assert!(response.cqc_hdr.msg_type.is_done(), "Unexpected response");
//! }
//!
//! // Receive the qubit on the remote node, `localhost:8804`.
//! {
//!     // Open connection to local node.
//!     let stream = net::TcpStream::connect((hostname.as_str(), remote_port))
//!         .expect("Connect failed");
//!
//!     // Send a request to receive a qubit.
//!     let request = client.cmd_recv(0, hdr::CmdOpt::empty());
//!     coder.serialize_into(&stream, &request).expect(
//!         "Sending failed",
//!     );
//!
//!     // Receive a response.
//!     let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");
//!     assert!(response.cqc_hdr.msg_type.is_recv(), "Unexpected response");
//!     let note = response.notify.get_qubit_hdr();
//!     let qubit_id = note.qubit_id;
//!
//!     println!("Received qubit ID: {}", qubit_id);
//! }
//! ```

extern crate bincode;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_display_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod builder;
pub mod hdr;

use hdr::*;

use self::serde::de;
use std::fmt;

use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;

// ----------------------------------------------------------------------------
// Macros.
// ----------------------------------------------------------------------------

macro_rules! def_is_hdr {
    ($enum_name: ident, $variant: ident, $fn_name: ident) => {
        pub fn $fn_name(&self) -> bool {
            match self {
                &$enum_name::$variant(_) => true,
                _ => false,
            }
        }
    }
}

macro_rules! def_get_hdr {
    ($enum_name: ident,
     $variant: ident,
     $return: ident,
     $fn_name: ident,
     $str_name: expr) => {
        pub fn $fn_name(self) -> $return {
            match self {
                $enum_name::$variant(x) => x,
                _ => panic!("Expected {}", $str_name),
            }
        }
    }
}

macro_rules! de_check_len {
    ($name: expr, $length: expr, $min: expr) => {
        if $length < $min {
            return Err(de::Error::invalid_length($length as usize, &$name));
        }
    };
}

macro_rules! de_hdr {
    ($seq: ident) => {
        $seq.next_element()?.unwrap()
    };
}

/// # Request
///
/// A valid CQC request will always begin with the CQC header.  A command
/// header must follow for certain message types.
#[derive(Debug, PartialEq)]
pub struct Request {
    pub cqc_hdr: CqcHdr,
    pub req_cmd: Option<ReqCmd>,
}

impl Request {
    pub fn len(&self) -> u32 {
        CqcHdr::hdr_len()
            + self.req_cmd.as_ref().map(|hdr| hdr.len()).unwrap_or(0)
    }
}

/// # Command Request
///
/// A command request follows the CQC Header for certain message types.  It
/// consists of the Command Header and for certain command types an additional
/// header is required.
#[derive(Debug, PartialEq)]
pub struct ReqCmd {
    pub cmd_hdr: CmdHdr,
    pub xtra_hdr: XtraHdr,
}

impl ReqCmd {
    pub fn len(&self) -> u32 {
        CmdHdr::hdr_len() + self.xtra_hdr.len()
    }
}

/// # Extra Header
///
/// Some commands require an additional header to follow the Command Header.
#[derive(Debug, PartialEq)]
pub enum XtraHdr {
    Rot(RotHdr),
    Qubit(QubitHdr),
    Comm(CommHdr),
    None,
}

impl XtraHdr {
    pub fn len(&self) -> u32 {
        match *self {
            XtraHdr::Rot(_) => RotHdr::hdr_len(),
            XtraHdr::Qubit(_) => QubitHdr::hdr_len(),
            XtraHdr::Comm(_) => CommHdr::hdr_len(),
            XtraHdr::None => 0,
        }
    }

    def_is_hdr!(XtraHdr, Rot, is_rot_hdr);
    def_is_hdr!(XtraHdr, Qubit, is_qubit_hdr);
    def_is_hdr!(XtraHdr, Comm, is_comm_hdr);

    def_get_hdr!(XtraHdr, Rot, RotHdr, get_rot_hdr, "RotHdr");
    def_get_hdr!(XtraHdr, Qubit, QubitHdr, get_qubit_hdr, "QubitHdr");
    def_get_hdr!(XtraHdr, Comm, CommHdr, get_comm_hdr, "CommHdr");

    pub fn is_some(&self) -> bool {
        match self {
            &XtraHdr::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            &XtraHdr::None => true,
            _ => false,
        }
    }
}

// ----------------------------------------------------------------------------
// Request serialisation.
// ----------------------------------------------------------------------------

impl Serialize for Request {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Request", 2)?;
        s.serialize_field("CqcHdr", &self.cqc_hdr)?;
        if self.req_cmd.is_some() {
            s.serialize_field("ReqCmd", self.req_cmd.as_ref().unwrap())?;
        }
        s.end()
    }
}

impl Serialize for ReqCmd {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReqCmd", 2)?;
        s.serialize_field("CmdHdr", &self.cmd_hdr)?;
        match self.xtra_hdr {
            XtraHdr::Rot(ref h) => s.serialize_field("RotHdr", h)?,
            XtraHdr::Qubit(ref h) => s.serialize_field("QubtiHdr", h)?,
            XtraHdr::Comm(ref h) => s.serialize_field("CommHdr", h)?,
            XtraHdr::None => (),
        };
        s.end()
    }
}

// ----------------------------------------------------------------------------
// Request deserialisation.
// ----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Request {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Request, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] =
            &["CqcHdr", "CmdHdr", "XtraHdr"];
        deserializer.deserialize_struct("Request", FIELDS, RequestVisitor)
    }
}

struct RequestVisitor;

impl<'de> Visitor<'de> for RequestVisitor {
    type Value = Request;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a CQC request packet")
    }

    #[inline]
    fn visit_seq<V>(self, mut seq: V) -> Result<Request, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let cqc_hdr: CqcHdr = de_hdr!(seq);
        let (msg_type, length) = (cqc_hdr.msg_type, cqc_hdr.length);

        if length == 0 {
            return Ok(Request {
                cqc_hdr,
                req_cmd: None,
            });
        }

        let req_cmd = match msg_type {
            MsgType::Tp(Tp::Hello) => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Other(
                        "Hello message should not have a message body",
                    ),
                    &self,
                ));
            }

            MsgType::Tp(Tp::GetTime) | MsgType::Tp(Tp::Command) => {
                de_check_len!("CmdHdr", length, CmdHdr::hdr_len());
                let cmd_hdr: CmdHdr = de_hdr!(seq);

                let length = length - CmdHdr::hdr_len();
                let xtra_hdr = match cmd_hdr.instr {
                    Cmd::RotX | Cmd::RotY | Cmd::RotZ => {
                        de_check_len!("RotHdr", length, RotHdr::hdr_len());
                        XtraHdr::Rot(de_hdr!(seq))
                    }

                    Cmd::Cnot | Cmd::Cphase => {
                        de_check_len!("QubitHdr", length, QubitHdr::hdr_len());
                        XtraHdr::Qubit(de_hdr!(seq))
                    }

                    Cmd::Send | Cmd::Epr => {
                        de_check_len!("CommHdr", length, CommHdr::hdr_len());
                        XtraHdr::Comm(de_hdr!(seq))
                    }

                    _ => XtraHdr::None,
                };

                Some(ReqCmd { cmd_hdr, xtra_hdr })
            }

            MsgType::Tp(Tp::Factory)
            | MsgType::Tp(Tp::InfTime)
            | MsgType::Tp(Tp::Mix)
            | MsgType::Tp(Tp::If) => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Other(
                        &vec![
                            "Deserialise not yet supported for:".to_string(),
                            msg_type.to_string(),
                        ]
                        .join(" "),
                    ),
                    &self,
                ));
            }

            _ => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Other(
                        &vec![
                            "Unexpected message type:".to_string(),
                            msg_type.to_string(),
                        ]
                        .join(" "),
                    ),
                    &self,
                ));
            }
        };

        Ok(Request { cqc_hdr, req_cmd })
    }
}

/// # Response
///
/// If the notify flag is set on a request, the CQC Backend will return a
/// response.  It begins with a CQC Header followed by either a Notify Header
/// or an Entanglement Information Header.
#[derive(Debug, PartialEq)]
pub struct Response {
    pub cqc_hdr: CqcHdr,
    pub notify: RspInfo,
}

impl Response {
    pub fn len(&self) -> u32 {
        CqcHdr::hdr_len() + self.notify.len()
    }
}

/// # Response Info
///
/// Some responses from a CQC backed will be followed by either a Notify Header
/// or an Entanglement Info Header.
#[derive(Debug, PartialEq)]
pub enum RspInfo {
    Qubit(QubitHdr),
    MeasOut(MeasOutHdr),
    Epr(EprInfo),
    Time(TimeInfoHdr),
    None,
}

impl RspInfo {
    pub fn len(&self) -> u32 {
        match *self {
            RspInfo::Qubit(_) => QubitHdr::hdr_len(),
            RspInfo::MeasOut(_) => MeasOutHdr::hdr_len(),
            RspInfo::Epr(_) => QubitHdr::hdr_len() + EntInfoHdr::hdr_len(),
            RspInfo::Time(_) => TimeInfoHdr::hdr_len(),
            RspInfo::None => 0,
        }
    }

    def_is_hdr!(RspInfo, Qubit, is_qubit_hdr);
    def_is_hdr!(RspInfo, MeasOut, is_meas_out_hdr);
    def_is_hdr!(RspInfo, Epr, is_epr_hdr);
    def_is_hdr!(RspInfo, Time, is_time_info_hdr);

    def_get_hdr!(RspInfo, Qubit, QubitHdr, get_qubit_hdr, "QubitHdr");
    def_get_hdr!(RspInfo, MeasOut, MeasOutHdr, get_meas_out_hdr, "MeasOutHdr");
    def_get_hdr!(RspInfo, Epr, EprInfo, get_epr_hdr, "EprInfo");
    def_get_hdr!(RspInfo, Time, TimeInfoHdr, get_time_info_hdr, "TimeInfoHdr");

    pub fn is_some(&self) -> bool {
        match self {
            &RspInfo::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            &RspInfo::None => true,
            _ => false,
        }
    }
}

/// # EPR Info
///
/// A response about an EPR pair consists of an Extra Qubit header and an
/// Entanglement Information header
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EprInfo {
    pub qubit_hdr: QubitHdr,
    pub ent_info_hdr: EntInfoHdr,
}

// ----------------------------------------------------------------------------
// Response serialisation.
// ----------------------------------------------------------------------------

impl Serialize for Response {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 2)?;
        s.serialize_field("CqcHdr", &self.cqc_hdr)?;
        if self.notify.is_some() {
            s.serialize_field("RspInfo", &self.notify)?;
        }
        s.end()
    }
}

impl Serialize for RspInfo {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("RspInfo", 1)?;
        match self {
            RspInfo::Qubit(ref h) => s.serialize_field("QubtiHdr", h)?,
            RspInfo::MeasOut(ref h) => s.serialize_field("MeasOutHdr", h)?,
            RspInfo::Epr(ref h) => s.serialize_field("EprInfo", h)?,
            RspInfo::Time(ref h) => s.serialize_field("TimeInfoHdr", h)?,
            RspInfo::None => (),
        };
        s.end()
    }
}

// ----------------------------------------------------------------------------
// Response deserialisation.
// ----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Response {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Response, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["CqcHdr", "Notify"];
        deserializer.deserialize_struct("Response", FIELDS, ResponseVisitor)
    }
}

struct ResponseVisitor;

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a CQC response packet")
    }

    #[inline]
    fn visit_seq<V>(self, mut seq: V) -> Result<Response, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let cqc_hdr: CqcHdr = de_hdr!(seq);
        let (msg_type, length) = (cqc_hdr.msg_type, cqc_hdr.length);

        if length == 0 {
            return Ok(Response {
                cqc_hdr,
                notify: RspInfo::None,
            });
        }

        let notify = match msg_type {
            MsgType::Tp(Tp::Recv) | MsgType::Tp(Tp::NewOk) => {
                de_check_len!("QubitHdr", length, QubitHdr::hdr_len());
                RspInfo::Qubit(de_hdr!(seq))
            }

            MsgType::Tp(Tp::MeasOut) => {
                de_check_len!("MeasOutHdr", length, MeasOutHdr::hdr_len());
                RspInfo::MeasOut(de_hdr!(seq))
            }

            MsgType::Tp(Tp::InfTime) => {
                de_check_len!("TimeInfoHdr", length, TimeInfoHdr::hdr_len());
                RspInfo::Time(de_hdr!(seq))
            }

            MsgType::Tp(Tp::EprOk) => {
                de_check_len!(
                    "QubitHdr + EntInfoHdr",
                    length,
                    QubitHdr::hdr_len() + EntInfoHdr::hdr_len()
                );
                RspInfo::Epr(de_hdr!(seq))
            }

            _ => RspInfo::None,
        };

        Ok(Response { cqc_hdr, notify })
    }
}

/// # Packet encoder
///
/// A basic packet encoder
pub struct Encoder {
    config: bincode::Config,
}

impl Encoder {
    /// Create a big endian `Encoder`.
    pub fn new() -> Encoder {
        let mut config = bincode::config();
        config.big_endian();

        Encoder { config }
    }

    /// Encode a CQC request packet into buffer of bytes.
    ///
    /// If the provided buffer is not large enough to encode the request
    /// `encode_request` will panic.
    pub fn encode<'buf, T>(&self, request: &T, buffer: &'buf mut [u8])
    where
        T: Serialize,
    {
        self.config
            .serialize_into(&mut buffer[..], &request)
            .unwrap();
    }

    /// Encode a CQC request packet into a newly allocated vector of bytes.
    pub fn into_vec<T>(&self, request: &T) -> Vec<u8>
    where
        T: Serialize,
    {
        self.config.serialize(&request).unwrap()
    }
}

/// # Packet decoder
///
/// A basic packet decoder.
pub struct Decoder {
    config: bincode::Config,
}

impl Decoder {
    /// Create a big endian `Decoder`.
    pub fn new() -> Decoder {
        let mut config = bincode::config();
        config.big_endian();

        Decoder { config }
    }

    /// Decode supplied data.
    ///
    /// Returns a Result which contains either the Response or an error.
    pub fn decode<T>(&self, buffer: &[u8]) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let response = self.config.deserialize_from(buffer)?;
        Ok(response)
    }
}
