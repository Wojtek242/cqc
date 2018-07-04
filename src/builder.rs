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

/// Build a command request.
pub fn build_cmd(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }
/// Build a factory request.
pub fn build_factory(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }
/// Build a qubit creation time query.
pub fn build_get_time(app_id: u16, req_cmd: ReqCmd) -> Request { panic!() }

/// Build an identity operation command request.
pub fn build_cmd_i(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a qubit creation command request.
pub fn build_cmd_new(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a measurement command request.
pub fn build_cmd_measure(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build an in-place measurement command request.
pub fn build_cmd_measure_inplace(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a reset command request.
pub fn build_cmd_reset(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a send command request.  This command requires an Xtra Header to identify the remote node.
pub fn build_cmd_send(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build a receive command request.  This command requires an Xtra Header to identify the remote node.
pub fn build_cmd_recv(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build an EPR creation command request.
pub fn build_cmd_epr(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build an EPR receive command request.
pub fn build_cmd_epr_recv(qubit_id: u16, options: u8) -> ReqCmd { panic!() }

/// Build a Pauli X command request.
pub fn build_cmd_x(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a Pauli Z command request.
pub fn build_cmd_z(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a Pauli Y command request.
pub fn build_cmd_y(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a T Gate command request.
pub fn build_cmd_t(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build an X rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn build_cmd_rot_x(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build a Y rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn build_cmd_rot_y(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build a Z rotation command request.  This command requires an Xtra Header to specify the angle of rotation.
pub fn build_cmd_rot_z(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build a Hadamard Gate command request.
pub fn build_cmd_h(qubit_id: u16, options: u8) -> ReqCmd { panic!() }
/// Build a K Gate command request.
pub fn build_cmd_k(qubit_id: u16, options: u8) -> ReqCmd { panic!() }

/// Build a CNOT command request.  This command requires an Xtra Header to identify the target qubit.
pub fn build_cmd_cnot(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }
/// Build a CPHASE command request.  This command requires an Xtra Header to identify the target qubit.
pub fn build_cmd_cphase(qubit_id: u16, options: u8, xtra_hdr: XtraHdr) -> ReqCmd { panic!() }

/// Build an Xtra Header.
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
