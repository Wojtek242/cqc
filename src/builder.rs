//! # CQC Packet builder
//!
//! This module provides utility functions to build valid CQC packets.  Its
//! main purpose is to provide an API that can guarantee correct packet format.
//! It is not necessary to use the `builder` module to build CQC packets, but
//! packets built exclusively with this API are guaranteed to be correct.
//!
//! Currently only client-to-server packets are supported.

use hdr::*;
use {ReqCmd, Request, XtraHdr};

/// Struct containing all the necessary bits of information to identify a
/// remote instance of the CQC backend.
#[derive(Copy, Clone)]
pub struct RemoteId {
    remote_app_id: u16,
    remote_node: u32,
    remote_port: u16,
}

impl Request {
    /// Build a CQC header
    fn build_cqc_hdr(app_id: u16, msg_type: MsgType) -> Request {
        let cqc_hdr = CqcHdr {
            version: Version::V1,
            msg_type: msg_type,
            app_id: app_id,
            length: 0,
        };

        Request {
            cqc_hdr,
            req_cmd: None,
        }
    }

    /// Build a liveness check request.
    pub fn hello(app_id: u16) -> Request {
        Request::build_cqc_hdr(app_id, MsgType::Tp(Tp::Hello))
    }

    /// Build a qubit creation time query.
    pub fn get_time(app_id: u16, qubit_id: u16) -> Request {
        let mut request = Request::build_cqc_hdr(app_id, MsgType::Tp(Tp::GetTime));
        request.cmd_i(qubit_id, CmdOpt::empty());
        request
    }

    /// Build a command request.
    pub fn command(app_id: u16) -> Request {
        Request::build_cqc_hdr(app_id, MsgType::Tp(Tp::Command))
    }

    /// Build an identity operation command request.
    pub fn cmd_i(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::I, options, XtraHdr::None);
    }
    /// Build a qubit creation command request.
    pub fn cmd_new(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::New, options, XtraHdr::None);
    }
    /// Build a measurement command request.
    pub fn cmd_measure(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Measure, options, XtraHdr::None);
    }
    /// Build an in-place measurement command request.
    pub fn cmd_measure_inplace(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::MeasureInplace, options, XtraHdr::None);
    }
    /// Build a reset command request.
    pub fn cmd_reset(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Reset, options, XtraHdr::None);
    }
    /// Build a send command request.  This command has to identify the remote node to send to.
    pub fn cmd_send(&mut self, qubit_id: u16, options: CmdOpt, remote_id: RemoteId) {
        let xtra_hdr = Request::xtra_remote_node(remote_id);
        self.build_req_cmd(qubit_id, Cmd::Send, options, xtra_hdr);
    }
    /// Build a receive command request.
    pub fn cmd_recv(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Recv, options, XtraHdr::None);
    }
    /// Build an EPR creation command request.
    pub fn cmd_epr(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Epr, options, XtraHdr::None);
    }
    /// Build an EPR receive command request.
    pub fn cmd_epr_recv(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::EprRecv, options, XtraHdr::None);
    }

    /// Build a Pauli X command request.
    pub fn cmd_x(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::X, options, XtraHdr::None);
    }
    /// Build a Pauli Z command request.
    pub fn cmd_z(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Z, options, XtraHdr::None);
    }
    /// Build a Pauli Y command request.
    pub fn cmd_y(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::Y, options, XtraHdr::None);
    }
    /// Build a T Gate command request.
    pub fn cmd_t(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::T, options, XtraHdr::None);
    }
    /// Build an X rotation command request.  Rotation is specified in steps of pi/256 increments.
    pub fn cmd_rot_x(&mut self, qubit_id: u16, options: CmdOpt, steps: u8) {
        let xtra_hdr = Request::xtra_rotation_angle(steps);
        self.build_req_cmd(qubit_id, Cmd::RotX, options, xtra_hdr);
    }
    /// Build a Y rotation command request.  Rotation is specified in steps of pi/256 increments.
    pub fn cmd_rot_y(&mut self, qubit_id: u16, options: CmdOpt, steps: u8) {
        let xtra_hdr = Request::xtra_rotation_angle(steps);
        self.build_req_cmd(qubit_id, Cmd::RotY, options, xtra_hdr);
    }
    /// Build a Z rotation command request.  Rotation is specified in steps of pi/256 increments.
    pub fn cmd_rot_z(&mut self, qubit_id: u16, options: CmdOpt, steps: u8) {
        let xtra_hdr = Request::xtra_rotation_angle(steps);
        self.build_req_cmd(qubit_id, Cmd::RotZ, options, xtra_hdr);
    }
    /// Build a Hadamard Gate command request.
    pub fn cmd_h(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::H, options, XtraHdr::None);
    }
    /// Build a K Gate command request.
    pub fn cmd_k(&mut self, qubit_id: u16, options: CmdOpt) {
        self.build_req_cmd(qubit_id, Cmd::K, options, XtraHdr::None);
    }

    /// Build a CNOT command request.  This command requires an Xtra Header to identify the target qubit.
    pub fn cmd_cnot(&mut self, qubit_id: u16, options: CmdOpt, target_qubit_id: u16) {
        let xtra_hdr = Request::xtra_target_qubit(target_qubit_id);
        self.build_req_cmd(qubit_id, Cmd::Cnot, options, xtra_hdr);
    }
    /// Build a CPHASE command request.
    pub fn cmd_cphase(&mut self, qubit_id: u16, options: CmdOpt, target_qubit_id: u16) {
        let xtra_hdr = Request::xtra_target_qubit(target_qubit_id);
        self.build_req_cmd(qubit_id, Cmd::Cphase, options, xtra_hdr);
    }

    /// Build a Command Header Request.
    pub fn build_req_cmd(
        &mut self,
        qubit_id: u16,
        instr: Cmd,
        options: CmdOpt,
        xtra_hdr: XtraHdr,
    ) {
        let cmd_hdr = CmdHdr {
            qubit_id,
            instr,
            options,
        };

        self.append_req_cmd(ReqCmd { cmd_hdr, xtra_hdr });
    }

    /// Append a Command Header request.
    fn append_req_cmd(&mut self, req_cmd: ReqCmd) {
        self.cqc_hdr.length = CMD_HDR_LENGTH + req_cmd.xtra_hdr.len();
        self.req_cmd = Some(req_cmd);
    }

    /// Build an Xtra Header that specifies a remote node.
    fn xtra_remote_node(remote_id: RemoteId) -> XtraHdr {
        XtraHdr::Comm(CommHdr {
            remote_app_id: remote_id.remote_app_id,
            remote_node: remote_id.remote_node,
            remote_port: remote_id.remote_port,
        })
    }

    /// Build an Xtra Header that specifies a rotation angle in pi/256 increments.
    fn xtra_rotation_angle(step: u8) -> XtraHdr {
        XtraHdr::Rot(RotHdr { step })
    }

    /// Build an Xtra Header that specifies a target qubit.
    fn xtra_target_qubit(qubit_id: u16) -> XtraHdr {
        XtraHdr::Qubit(QubitHdr { qubit_id })
    }
}
