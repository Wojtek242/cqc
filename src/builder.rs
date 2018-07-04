//! # CQC Packet builder
//!
//! This module provides utility functions to build valid CQC packets.  Its
//! main purpose is to provide an API that can guarantee correct packet format.
//! CQC packets built exclusively with this API are guaranteed to be correct.

use hdr::*;
use {ReqCmd, Request};

pub trait SetOpts {
    fn set_opt_notify(self) -> u8;
    fn set_opt_action(self) -> u8;
    fn set_opt_block(self) -> u8;
    fn set_opt_ifthen(self) -> u8;
}

impl SetOpts for u8 {
    #[inline]
    fn set_opt_notify(self) -> u8 {
        self | CMD_OPT_NOTIFY
    }

    #[inline]
    fn set_opt_action(self) -> u8 {
        self | CMD_OPT_ACTION
    }

    #[inline]
    fn set_opt_block(self) -> u8 {
        self | CMD_OPT_BLOCK
    }

    #[inline]
    fn set_opt_ifthen(self) -> u8 {
        self | CMD_OPT_IFTHEN
    }
}

pub fn build_hello(app_id: u16) -> Request {
    let cqc_hdr = CqcHdr {
        version: CQC_VERSION,
        msg_type: MsgType::Tp(CqcTp::Hello),
        app_id: app_id,
        length: 0,
    };

    Request {
        cqc_hdr,
        command: None,
    }
}

pub fn build_cmd(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }
pub fn build_factory(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }
pub fn build_get_time(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }

pub fn build_cmd_i(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_new(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_measure(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_measure_inplace(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_reset(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_send(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_recv(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_epr(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_epr_recv(qubit_id: u16, options: u8) -> ReqCmd { panic!() }

pub fn build_cmd_x(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_z(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_y(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_t(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_rot_x(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_rot_y(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_rot_z(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_h(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
pub fn build_cmd_k(qubit_id: u16, options: u8) -> ReqCmd { panic!() }

pub fn build_cmd_cnot(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
pub fn build_cmd_cphase(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }

pub fn build_xtra_hdr(
    xtra_qubit_id: u16,
    remote_app_id: u16,
    remote_node: u32,
    cmd_length: u32,
    remote_port: u16,
    setps: u8,
) -> XtraHdr {
    panic!()
}
