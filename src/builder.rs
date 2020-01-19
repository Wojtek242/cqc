//! # CQC Packet builder
//!
//! This module provides utility functions to build valid CQC packets.  Its
//! main purpose is to provide an API that can guarantee correct packet format.
//! It is not necessary to use the `builder` module to build CQC packets, but
//! packets built exclusively with this API are guaranteed to be correct.
//!
//! Currently only client-to-server packets are supported.
//!
//! This module is to be used in conjunction with the CQC interface
//! documentation available in the `hdr` module.

use hdr::*;
use {ReqCmd, Request, XtraHdr, RspInfo, Response, EprInfo};

/// Struct containing all the necessary bits of information to identify a
/// remote instance of the CQC backend.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoteId {
    pub remote_app_id: u16,
    pub remote_node: u32,
    pub remote_port: u16,
}

/// The Client builder constructs requests for a particular application ID.
pub struct Client {
    app_id: u16,
}

impl Client {
    /// Construct a Client builder.
    #[inline]
    pub fn new(app_id: u16) -> Self {
        Client { app_id }
    }

    /// Build a basic CQC request.
    fn build(&self, msg_type: MsgType, req_cmd: Option<ReqCmd>) -> Request {
        let cqc_hdr = CqcHdr {
            version: Version::V2,
            msg_type: msg_type,
            app_id: self.app_id,
            length: match req_cmd {
                Some(ref req) => req.len(),
                None => 0,
            },
        };

        Request { cqc_hdr, req_cmd }
    }

    /// Build a liveness check request.
    #[inline]
    pub fn hello(&self) -> Request {
        self.build(MsgType::Tp(Tp::Hello), None)
    }

    /// Build a qubit creation time query.
    #[inline]
    pub fn get_time(&self, qubit_id: u16) -> Request {
        let req_cmd = self.build_req_cmd(qubit_id, Cmd::I, CmdOpt::empty(), XtraHdr::None);
        self.build(MsgType::Tp(Tp::GetTime), Some(req_cmd))
    }

    /// Build a command request.
    fn command(&self, req_cmd: ReqCmd) -> Request {
        self.build(MsgType::Tp(Tp::Command), Some(req_cmd))
    }

    /// Build an identity operation command request.
    #[inline]
    pub fn cmd_i(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::I, options, XtraHdr::None))
    }
    /// Build a qubit creation command request.
    #[inline]
    pub fn cmd_new(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::New, options, XtraHdr::None))
    }
    /// Build a measurement command request.
    #[inline]
    pub fn cmd_measure(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::Measure, options, XtraHdr::None))
    }
    /// Build an in-place measurement command request.
    #[inline]
    pub fn cmd_measure_inplace(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::MeasureInplace, options, XtraHdr::None))
    }
    /// Build a reset command request.
    #[inline]
    pub fn cmd_reset(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::Reset, options, XtraHdr::None))
    }
    /// Build a send command request.  This command has to identify the remote node to send to.
    #[inline]
    pub fn cmd_send(&self, qubit_id: u16, options: CmdOpt, remote_id: RemoteId) -> Request {
        let xtra_hdr = self.xtra_remote_node(remote_id);
        self.command(self.build_req_cmd(qubit_id, Cmd::Send, options, xtra_hdr))
    }
    /// Build a receive command request.
    #[inline]
    pub fn cmd_recv(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::Recv, options, XtraHdr::None))
    }
    /// Build an EPR creation command request.
    #[inline]
    pub fn cmd_epr(&self, qubit_id: u16, options: CmdOpt, remote_id: RemoteId) -> Request {
        let xtra_hdr = self.xtra_remote_node(remote_id);
        self.command(self.build_req_cmd(qubit_id, Cmd::Epr, options, xtra_hdr))
    }
    /// Build an EPR receive command request.
    #[inline]
    pub fn cmd_epr_recv(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::EprRecv, options, XtraHdr::None))
    }

    /// Build a Pauli X command request.
    #[inline]
    pub fn cmd_x(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::X, options, XtraHdr::None))
    }
    /// Build a Pauli Z command request.
    #[inline]
    pub fn cmd_z(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::Z, options, XtraHdr::None))
    }
    /// Build a Pauli Y command request.
    #[inline]
    pub fn cmd_y(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::Y, options, XtraHdr::None))
    }
    /// Build a T Gate command request.
    #[inline]
    pub fn cmd_t(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::T, options, XtraHdr::None))
    }
    /// Build an X rotation command request.  Rotation is specified in steps of pi/256 increments.
    #[inline]
    pub fn cmd_rot_x(&self, qubit_id: u16, options: CmdOpt, steps: u8) -> Request {
        let xtra_hdr = self.xtra_rotation_angle(steps);
        self.command(self.build_req_cmd(qubit_id, Cmd::RotX, options, xtra_hdr))
    }
    /// Build a Y rotation command request.  Rotation is specified in steps of pi/256 increments.
    #[inline]
    pub fn cmd_rot_y(&self, qubit_id: u16, options: CmdOpt, steps: u8) -> Request {
        let xtra_hdr = self.xtra_rotation_angle(steps);
        self.command(self.build_req_cmd(qubit_id, Cmd::RotY, options, xtra_hdr))
    }
    /// Build a Z rotation command request.  Rotation is specified in steps of pi/256 increments.
    #[inline]
    pub fn cmd_rot_z(&self, qubit_id: u16, options: CmdOpt, steps: u8) -> Request {
        let xtra_hdr = self.xtra_rotation_angle(steps);
        self.command(self.build_req_cmd(qubit_id, Cmd::RotZ, options, xtra_hdr))
    }
    /// Build a Hadamard Gate command request.
    #[inline]
    pub fn cmd_h(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::H, options, XtraHdr::None))
    }
    /// Build a K Gate command request.
    #[inline]
    pub fn cmd_k(&self, qubit_id: u16, options: CmdOpt) -> Request {
        self.command(self.build_req_cmd(qubit_id, Cmd::K, options, XtraHdr::None))
    }

    /// Build a CNOT command request.  Requires a target qubit.
    #[inline]
    pub fn cmd_cnot(&self, qubit_id: u16, options: CmdOpt, target_qubit_id: u16) -> Request {
        let xtra_hdr = self.xtra_target_qubit(target_qubit_id);
        self.command(self.build_req_cmd(qubit_id, Cmd::Cnot, options, xtra_hdr))
    }
    /// Build a CPHASE command request.  Requires a target qubit.
    #[inline]
    pub fn cmd_cphase(&self, qubit_id: u16, options: CmdOpt, target_qubit_id: u16) -> Request {
        let xtra_hdr = self.xtra_target_qubit(target_qubit_id);
        self.command(self.build_req_cmd(qubit_id, Cmd::Cphase, options, xtra_hdr))
    }

    /// Build a Command Header Request.
    fn build_req_cmd(
        &self,
        qubit_id: u16,
        instr: Cmd,
        options: CmdOpt,
        xtra_hdr: XtraHdr,
    ) -> ReqCmd {
        let cmd_hdr = CmdHdr {
            qubit_id,
            instr,
            options,
        };

        ReqCmd { cmd_hdr, xtra_hdr }
    }

    /// Build an Xtra Header that specifies a remote node.
    fn xtra_remote_node(&self, remote_id: RemoteId) -> XtraHdr {
        XtraHdr::Comm(CommHdr {
            remote_app_id: remote_id.remote_app_id,
            remote_node: remote_id.remote_node,
            remote_port: remote_id.remote_port,
        })
    }

    /// Build an Xtra Header that specifies a rotation angle in pi/256 increments.
    fn xtra_rotation_angle(&self, step: u8) -> XtraHdr {
        XtraHdr::Rot(RotHdr { step })
    }

    /// Build an Xtra Header that specifies a target qubit.
    fn xtra_target_qubit(&self, qubit_id: u16) -> XtraHdr {
        XtraHdr::Qubit(QubitHdr { qubit_id })
    }
}

pub struct Server {
    app_id: u16,
}

impl Server {
    /// Construct a Server builder.
    #[inline]
    pub fn new(app_id: u16) -> Self {
        Server { app_id }
    }

    /// Build a basic CQC response.
    fn build(&self, msg_type: MsgType, notify: RspInfo) -> Response {
        let cqc_hdr = CqcHdr {
            version: Version::V2,
            msg_type: msg_type,
            app_id: self.app_id,
            length: notify.len(),
        };

        Response { cqc_hdr, notify }
    }

    /// Build an Expire notification.
    #[inline]
    pub fn expire(&self, qubit_id: u16) -> Response {
        let notify = self.rsp_info_qubit(qubit_id);
        self.build(MsgType::Tp(Tp::Expire), notify)
    }
    /// Build a Done notification.
    #[inline]
    pub fn done(&self) -> Response {
        self.build(MsgType::Tp(Tp::Done), RspInfo::None)
    }
    /// Build a Recv message for a received qubit.
    #[inline]
    pub fn recv(&self, qubit_id: u16) -> Response {
        let notify = self.rsp_info_qubit(qubit_id);
        self.build(MsgType::Tp(Tp::Recv), notify)
    }
    /// Build an EPR message for an entangled pair.
    #[inline]
    pub fn epr_ok(&self, qubit_id: u16, ent_info: EntInfoHdr) -> Response {
        let notify = self.rsp_info_epr(qubit_id, ent_info);
        self.build(MsgType::Tp(Tp::EprOk), notify)
    }
    /// Build a MeasOut message for a measurement outcome.
    #[inline]
    pub fn meas_out(&self, meas_out: MeasOut) -> Response {
        let notify = self.rsp_info_meas_out(meas_out);
        self.build(MsgType::Tp(Tp::MeasOut), notify)
    }
    /// Build a InfTime message for time information.
    #[inline]
    pub fn inf_time(&self, datetime: u64) -> Response {
        let notify = self.rsp_info_time_info(datetime);
        self.build(MsgType::Tp(Tp::InfTime), notify)
    }
    /// Build a NewOk message for a new qubit.
    #[inline]
    pub fn new_ok(&self, qubit_id: u16) -> Response {
        let notify = self.rsp_info_qubit(qubit_id);
        self.build(MsgType::Tp(Tp::NewOk), notify)
    }

    /// Build an RspInfo message block for a qubit.
    fn rsp_info_qubit(&self, qubit_id: u16) -> RspInfo {
        RspInfo::Qubit(QubitHdr { qubit_id })
    }

    /// Build an RspInfo message block for a measurement outcome.
    fn rsp_info_meas_out(&self, meas_out: MeasOut) -> RspInfo {
        RspInfo::MeasOut(MeasOutHdr { meas_out })
    }

    /// Build an RspInfo message block for entanglement.
    fn rsp_info_epr(&self, qubit_id: u16, ent_info: EntInfoHdr) -> RspInfo {
        RspInfo::Epr(
            EprInfo {
                qubit_hdr: QubitHdr { qubit_id },
                ent_info_hdr: ent_info,
            }
        )
    }

    /// Build an RspInfo message block for time information.
    fn rsp_info_time_info(&self, datetime: u64) -> RspInfo {
        RspInfo::Time(TimeInfoHdr { datetime })
    }
}
