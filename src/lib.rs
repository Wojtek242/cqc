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
//! let builder = builder::Builder::new(app_id);
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
//!     let request = builder.cmd_new(0, hdr::CmdOpt::empty());
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
//!     let request = builder.cmd_send(
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
//!     let request = builder.cmd_recv(0, hdr::CmdOpt::empty());
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

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

pub mod builder;
pub mod hdr;

use hdr::*;

use self::serde::de;
use std::fmt;

use serde::de::{SeqAccess, Visitor};
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
                _ => panic!("Expected {} Header", $str_name),
            }
        }
    }
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
            + match self.req_cmd {
                Some(ref r) => r.len(),
                None => 0,
            }
    }
}

impl Serialize for Request {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Request", 2)?;
        s.serialize_field("cqc_hdr", &self.cqc_hdr)?;
        if self.req_cmd.is_some() {
            s.serialize_field("req_cmd", self.req_cmd.as_ref().unwrap())?;
        }
        s.end()
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

impl Serialize for ReqCmd {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReqCmd", 2)?;
        s.serialize_field("cmd_hdr", &self.cmd_hdr)?;
        if self.xtra_hdr.is_some() {
            match self.xtra_hdr {
                XtraHdr::Rot(ref h) => s.serialize_field("xtra_hdr", h)?,
                XtraHdr::Qubit(ref h) => s.serialize_field("xtra_hdr", h)?,
                XtraHdr::Comm(ref h) => s.serialize_field("xtra_hdr", h)?,
                XtraHdr::None => panic!("Do not serialize XtraHdr::None"),
            };
        }
        s.end()
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

    def_get_hdr!(XtraHdr, Rot, RotHdr, get_rot_hdr, "Rotation");
    def_get_hdr!(XtraHdr, Qubit, QubitHdr, get_qubit_hdr, "Extra Qubit");
    def_get_hdr!(XtraHdr, Comm, CommHdr, get_comm_hdr, "Communication");

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

/// # Response Info
///
/// Some responses from a CQC backed will be followed by either a Notify Header
/// or an Entanglement Info Header.
#[derive(Debug, PartialEq)]
pub enum RspInfo {
    Qubit(QubitHdr),
    MeasOut(MeasOutHdr),
    Epr(EprInfo),
    None,
}

impl RspInfo {
    def_is_hdr!(RspInfo, Qubit, is_qubit_hdr);
    def_is_hdr!(RspInfo, MeasOut, is_meas_out_hdr);
    def_is_hdr!(RspInfo, Epr, is_epr_hdr);

    def_get_hdr!(RspInfo, Qubit, QubitHdr, get_qubit_hdr, "Qubit");
    def_get_hdr!(
        RspInfo,
        MeasOut,
        MeasOutHdr,
        get_meas_out_hdr,
        "Measurement Outcome"
    );
    def_get_hdr!(RspInfo, Epr, EprInfo, get_epr_hdr, "EPR");

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

struct ResponseVisitor;

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid CQC response packet")
    }

    #[inline]
    fn visit_seq<V>(self, mut seq: V) -> Result<Response, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let cqc_hdr: CqcHdr = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        if cqc_hdr.length == 0 {
            return Ok(Response {
                cqc_hdr,
                notify: RspInfo::None,
            });
        }

        let (msg_type, length) = (cqc_hdr.msg_type, cqc_hdr.length);
        let notify = match msg_type {
            MsgType::Tp(Tp::Recv) | MsgType::Tp(Tp::NewOk) => {
                if length < QubitHdr::hdr_len() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Other(
                            "Response length insufficient to hold an Extra Qubit Header",
                        ),
                        &self,
                    ));
                }
                let qubit_hdr: QubitHdr = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                RspInfo::Qubit(qubit_hdr)
            }
            MsgType::Tp(Tp::MeasOut) => {
                if length < MeasOutHdr::hdr_len() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Other(
                            "Response length insufficient to hold a Notify Header",
                        ),
                        &self,
                    ));
                }
                let meas_out_hdr: MeasOutHdr = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                RspInfo::MeasOut(meas_out_hdr)
            }
            MsgType::Tp(Tp::EprOk) => {
                if length < QubitHdr::hdr_len() + EntInfoHdr::hdr_len() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Other(
                            "Response length insufficient to hold an Entanglement Info Header",
                        ),
                        &self,
                    ));
                }
                let epr_info: EprInfo = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                RspInfo::Epr(epr_info)
            }
            _ => RspInfo::None,
        };

        Ok(Response { cqc_hdr, notify })
    }
}

impl<'de> Deserialize<'de> for Response {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Response, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["cqc_hdr", "notify"];
        deserializer.deserialize_struct("Response", FIELDS, ResponseVisitor)
    }
}

/// # EPR Info
///
/// A response about an EPR pair consists of an Extra Qubit header and an
/// Entanglement Information header
#[derive(Deserialize, Debug, PartialEq)]
pub struct EprInfo {
    pub qubit_hdr: QubitHdr,
    pub ent_info_hdr: EntInfoHdr,
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

    /// Encode a CQC request packet into buffer of bytes.  The return value is
    /// a the number of bytes written.
    ///
    /// If the provided buffer is not large enough to encode the request
    /// `encode_request` will panic.
    pub fn encode<'buf>(&self, request: &Request, buffer: &'buf mut [u8]) -> usize {
        let len = request.len() as usize;
        assert!(buffer.len() >= len);
        self.config
            .serialize_into(&mut buffer[..len], &request)
            .unwrap();

        len
    }

    /// Encode a CQC request packet into a newly allocated vector of bytes.
    pub fn into_vec(&self, request: &Request) -> Vec<u8> {
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
    pub fn decode(&self, buffer: &[u8]) -> Result<Response, Box<Error>> {
        let response = self.config.deserialize_from(buffer)?;
        Ok(response)
    }
}
