//! # CQC Encoder/Decoder
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

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;

pub mod hdr;
pub mod builder;
pub mod encode;
pub mod decode;

use hdr::*;

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

/// # Command Request
///
/// A command request follows the CQC Header for certain message types.  It
/// consists of the Command Header and for certain command types an additional
/// Xtra header is required.
#[derive(Debug, PartialEq)]
pub struct ReqCmd {
    pub cmd_hdr: CmdHdr,
    pub xtra_hdr: Option<XtraHdr>,
}

#[derive(Debug, PartialEq)]
pub enum XtraHdr {
    Rot(RotHdr),
    Qubit(QubitHdr),
    Comm(CommHdr),
    Factory(FactoryHdr),
}

impl XtraHdr {
    def_is_hdr!(XtraHdr, Rot, is_rot_hdr);
    def_is_hdr!(XtraHdr, Qubit, is_qubit_hdr);
    def_is_hdr!(XtraHdr, Comm, is_comm_hdr);
    def_is_hdr!(XtraHdr, Factory, is_factory_hdr);

    def_get_hdr!(XtraHdr, Rot, RotHdr, get_rot_hdr, "Rotation");
    def_get_hdr!(XtraHdr, Qubit, QubitHdr, get_qubit_hdr, "Extra Qubit");
    def_get_hdr!(XtraHdr, Comm, CommHdr, get_comm_hdr, "Communication");
    def_get_hdr!(XtraHdr, Factory, FactoryHdr, get_factory_hdr, "Factory");
}

/// # Response
///
/// If the notify flag is set on a request, the CQC Backend will return a
/// response.  It begins with a CQC Header followed by either a Notify Header
/// or an Entanglement Information Header.
#[derive(Debug, PartialEq)]
pub struct Response {
    pub cqc_hdr: CqcHdr,
    pub notify: Option<RspNotify>,
}

#[derive(Debug, PartialEq)]
pub enum RspNotify {
    Notify(NotifyHdr),
    EntInfo(EntInfoHdr),
}

impl RspNotify {
    def_is_hdr!(RspNotify, Notify, is_notify_hdr);
    def_is_hdr!(RspNotify, EntInfo, is_ent_info_hdr);

    def_get_hdr!(RspNotify, Notify, NotifyHdr, get_notify_hdr, "Notify");
    def_get_hdr!(
        RspNotify,
        EntInfo,
        EntInfoHdr,
        get_ent_info_hdr,
        "Entanglement Info"
    );
}
