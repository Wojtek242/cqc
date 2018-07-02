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

pub mod hdr;
pub mod encode;
pub mod decode;

use hdr::*;

/// # Request
///
/// A valid CQC request will always begin with the CQC header.  A command
/// header must follow for certain message types.

pub struct Request {
    pub cqc_hdr: CqcHdr,
    pub command: Option<ReqCmd>,
}

/// # Command Request
///
/// A command request follows the CQC Header for certain message types.  It
/// consists of the Command Header and additionally, certain command types
/// require an Xtra header.

pub struct ReqCmd {
    pub cmd_hdr: CmdHdr,
    pub xtra_hdr: Option<XtraHdr>,
}

/// # Response
///
/// If the notify flag is set on a request, the CQC Backend will return a
/// response.  It begins with a CQC Header followed by either a Notify Header
/// or an Entanglement Information Header.

pub struct Response {
    pub msg_hdr: CqcHdr,
    pub notify: RspNotify,
}

pub enum RspNotify {
    Notify(NotifyHdr),
    EntInfo(EntInfoHdr),
}
