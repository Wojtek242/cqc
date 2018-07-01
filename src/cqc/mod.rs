//! # CQC Interface
//!
//! This module documents the [CQC Interface
//! specification](https://stephaniewehner.github.io/SimulaQron/PreBetaDocs/CQCInterface.html)
//! and defines the necessary constants and structures.

pub mod hdr;
use self::hdr::*;

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
/// A command request consists of the Command Header and additionally, certain
/// command types require an Xtra header.

pub struct ReqCmd {
    pub cmd_hdr: CmdHdr,
    pub xtra_hdr: Option<XtraHdr>,
}

/// # Response
///
/// A valid CQC response will always begin with a CQC header.  If the notify
/// flag was set on the request, the response may also carry either a Notify
/// header or an Entanglement Information Header.

pub struct Response {
    pub msg_hdr: CqcHdr,
    pub notify: Option<RspNotify>,
}

pub enum RspNotify {
    Notify(NotifyHdr),
    EntInfo(EntInfoHdr),
}
