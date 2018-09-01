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
    pub fn is_notify_hdr(&self) -> bool {
        match self {
            &RspNotify::Notify(_) => true,
            &RspNotify::EntInfo(_) => false,
        }
    }

    pub fn get_notify_hdr(self) -> Option<NotifyHdr> {
        match self {
            RspNotify::Notify(notify_hdr) => Some(notify_hdr),
            RspNotify::EntInfo(_) => None,
        }
    }

    pub fn is_ent_info_hdr(&self) -> bool {
        match self {
            &RspNotify::Notify(_) => false,
            &RspNotify::EntInfo(_) => true,
        }
    }

    pub fn get_ent_info_hdr(self) -> Option<EntInfoHdr> {
        match self {
            RspNotify::Notify(_) => None,
            RspNotify::EntInfo(ent_info_hdr) => Some(ent_info_hdr),
        }
    }
}
