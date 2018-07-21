//! # CQC Packet builder
//!
//! This module provides utility functions to build valid CQC packets.  Its
//! main purpose is to provide an API that can guarantee correct packet format.
//! It is not necessary to use the `builder` module to build CQC packets, but
//! packets built exclusively with this API are guaranteed to be correct.
//!
//! Currently only client-to-server packets are supported.

use hdr::*;
use {ReqCmd, Request};

/// Convenience functions for setting bitwise options.
pub trait SetOpts {
    /// Convenience function to set the notify bit-flag.
    fn set_opt_notify(self) -> u8;
    /// Convenience function to set the action bit-flag.
    fn set_opt_action(self) -> u8;
    /// Convenience function to set the block bit-flag.
    fn set_opt_block(self) -> u8;
    /// Convenience function to set the if-then bit-flag.
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

/// Build a liveness check request.
pub fn hello(app_id: u16) -> Request {
    let cqc_hdr = CqcHdr {
        version: CQC_VERSION,
        msg_type: MsgType::Tp(CqcTp::Hello),
        app_id: app_id,
        length: 0,
    };

    Request {
        cqc_hdr,
        req_cmd: None,
    }
}

/// Build a command request.
pub fn command(app_id: u16, req_cmd: ReqCmd) -> Request {
    build_request(app_id, req_cmd, MsgType::Tp(CqcTp::Command))
}

/// Build a factory request.
pub fn factory(app_id: u16, req_cmd: ReqCmd) -> Request {
    build_request(app_id, req_cmd, MsgType::Tp(CqcTp::Factory))
}

/// Build a qubit creation time query.
pub fn get_time(app_id: u16, req_cmd: ReqCmd) -> Request {
    build_request(app_id, req_cmd, MsgType::Tp(CqcTp::GetTime))
}

/// Build a CQC Request.
fn build_request(app_id: u16, req_cmd: ReqCmd, msg_type: MsgType) -> Request {
    let cqc_hdr = CqcHdr {
        version: CQC_VERSION,
        msg_type: msg_type,
        app_id: app_id,
        length: CMD_HDR_LENGTH + if req_cmd.xtra_hdr.is_some() {
            XTRA_HDR_LENGTH
        } else {
            0
        },
    };

    Request {
        cqc_hdr,
        req_cmd: Some(req_cmd),
    }
}

/// Build an identity operation command request.
pub fn cmd_i(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::I)
}

/// Build a qubit creation command request.
pub fn cmd_new(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::New)
}
/// Build a measurement command request.
pub fn cmd_measure(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::Measure)
}
/// Build an in-place measurement command request.
pub fn cmd_measure_inplace(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::MeasureInplace)
}
/// Build a reset command request.
pub fn cmd_reset(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::Reset)
}
/// Build a send command request.  This command requires an Xtra Header to identify the remote node.
pub fn cmd_send(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::Send)
}
/// Build a receive command request.  This command requires an Xtra Header to identify the remote node.
pub fn cmd_recv(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::Recv)
}
/// Build an EPR creation command request.
pub fn cmd_epr(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::Epr)
}
/// Build an EPR receive command request.
pub fn cmd_epr_recv(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::EprRecv)
}

/// Build a Pauli X command request.
pub fn cmd_x(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::X)
}
/// Build a Pauli Z command request.
pub fn cmd_z(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::Z)
}
/// Build a Pauli Y command request.
pub fn cmd_y(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::Y)
}
/// Build a T Gate command request.
pub fn cmd_t(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::T)
}
/// Build an X rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn cmd_rot_x(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::RotX)
}
/// Build a Y rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn cmd_rot_y(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::RotY)
}
/// Build a Z rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn cmd_rot_z(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::RotZ)
}
/// Build a Hadamard Gate command request.
pub fn cmd_h(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::H)
}
/// Build a K Gate command request.
pub fn cmd_k(qubit_id: u16, options: u8) -> ReqCmd {
    build_req_cmd(qubit_id, options, None, Cmd::K)
}

/// Build a CNOT command request.  This command requires an Xtra Header to identify the target qubit.
pub fn cmd_cnot(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::Cnot)
}
/// Build a CPHASE command request.  This command requires an Xtra Header to identify the target qubit.
pub fn cmd_cphase(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd {
    build_req_cmd(qubit_id, options, Some(xtra_hdr), Cmd::Cphase)
}

/// Build a Command Header Request.
fn build_req_cmd(qubit_id: u16, options: u8, xtra_hdr: Option<XtraHdr>, instr: Cmd) -> ReqCmd {
    let cmd_hdr = CmdHdr {
        qubit_id,
        instr,
        options,
    };

    ReqCmd { cmd_hdr, xtra_hdr }
}

/// Build an Xtra Header.
pub fn build_xtra_hdr(
    xtra_qubit_id: u16,
    remote_app_id: u16,
    remote_node: u32,
    cmd_length: u32,
    remote_port: u16,
    steps: u8,
) -> XtraHdr {
    XtraHdr {
        xtra_qubit_id,
        remote_app_id,
        remote_node,
        cmd_length,
        remote_port,
        steps,
        align: 0,
    }
}
