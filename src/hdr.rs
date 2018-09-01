//! # CQC Interface
//!
//! This module documents the [CQC Interface
//! specification](https://stephaniewehner.github.io/SimulaQron/PreBetaDocs/CQCInterface.html)
//! and defines the necessary constants and header structures.

extern crate serde;

use self::serde::de;
use std::fmt;

use self::serde::de::Visitor;
use self::serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const CQC_VERSION: u8 = 0;

/// # CQC Header
///
/// Every CQC message begins with a CQC header.
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |    version    |    msg_type   |             app_id            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             length                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// Field     Length     Meaning
/// -----     ------     -------
/// version   1 byte     CQC interface version.  Current version is 0.
/// msg_type  1 byte     Message type.
/// app_id    2 bytes    Application ID.  Return messages will be tagged
///                      appropriately.
/// length    4 bytes    Total length of the CQC instruction packet.
/// ```
///
/// ## Possible Message Types
///
/// ```text
/// Type     Meaning
/// ----     -------
///  0       Alive check.
///  1       Execute a command list.
///  2       Start executing command list repeatedly.
///  3       Qubit has expired.
///  4       Command execution done.
///  5       Received qubit.
///  6       Created EPR pair.
///  7       Measurement outcome.
///  8       Get creation time of qubit.
///  9       Inform about time.
///  10      Created new qubit.
///
///  20      General purpose error (no details).
///  21      No more qubits available.
///  22      Command sequence not supported.
///  23      Timeout.
/// ```
///
/// A CQC Command Header MUST follow the CQC Header for the following messages:
///
///  - Execute a command list (msg_type=1).
///  - Start executing command list repeatedly (msg_type=2).
///  - Get creation time of qubit (msg_type=8).

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CqcHdr {
    pub version: u8,
    pub msg_type: MsgType,
    pub app_id: u16,
    pub length: u32,
}

pub const CQC_HDR_LENGTH: u32 = 8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MsgType {
    Tp(Tp),
    Err(Err),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tp {
    Hello = 0,   // Alive check.
    Command = 1, // Execute a command list.
    Factory = 2, // Start executing command list repeatedly.
    Expire = 3,  // Qubit has expired.
    Done = 4,    // Command execution done.
    Recv = 5,    // Recevied qubit.
    EprOk = 6,   // Created EPR pair.
    Measout = 7, // Measurement outcome.
    GetTime = 8, // Get creation time of qubit.
    InfTime = 9, // Inform about time.
    NewOk = 10,  // Created new qubit.
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Err {
    General = 20, // General purpose error (no details.
    Noqubit = 21, // No more qubits available.
    Unsupp = 22,  // Command sequence not supported.
    Timeout = 23, // Timeout.
}

impl MsgType {
    #[inline]
    pub fn is_tp(&self) -> bool {
        match self {
            &MsgType::Tp(_) => true,
            &MsgType::Err(_) => false,
        }
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        match self {
            &MsgType::Tp(_) => false,
            &MsgType::Err(_) => true,
        }
    }

    #[inline]
    pub fn is_hello(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Hello) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_command(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Command) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_factory(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Factory) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_expire(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Expire) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Done) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_recv(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Recv) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_epr_ok(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::EprOk) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_measout(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::Measout) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_get_time(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::GetTime) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_inf_time(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::InfTime) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_new_ok(&self) -> bool {
        match self {
            &MsgType::Tp(Tp::NewOk) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_err_general(&self) -> bool {
        match self {
            &MsgType::Err(Err::General) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_err_noqubit(&self) -> bool {
        match self {
            &MsgType::Err(Err::Noqubit) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_err_unsupp(&self) -> bool {
        match self {
            &MsgType::Err(Err::Unsupp) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_err_timeout(&self) -> bool {
        match self {
            &MsgType::Err(Err::Timeout) => true,
            _ => false,
        }
    }

    /// Convert an 8-bit value to a message type.  Returns `None` if the value
    /// does not correspond to a valid message type.
    #[inline]
    pub fn get_msg_type(value: u8) -> Option<MsgType> {
        let msg_type = match value {
            0 => MsgType::Tp(Tp::Hello),
            1 => MsgType::Tp(Tp::Command),
            2 => MsgType::Tp(Tp::Factory),
            3 => MsgType::Tp(Tp::Expire),
            4 => MsgType::Tp(Tp::Done),
            5 => MsgType::Tp(Tp::Recv),
            6 => MsgType::Tp(Tp::EprOk),
            7 => MsgType::Tp(Tp::Measout),
            8 => MsgType::Tp(Tp::GetTime),
            9 => MsgType::Tp(Tp::InfTime),
            10 => MsgType::Tp(Tp::NewOk),

            20 => MsgType::Err(Err::General),
            21 => MsgType::Err(Err::Noqubit),
            22 => MsgType::Err(Err::Unsupp),
            23 => MsgType::Err(Err::Timeout),

            _ => return None,
        };

        Some(msg_type)
    }
}

impl Serialize for MsgType {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &MsgType::Tp(tp) => serializer.serialize_u8(tp as u8),
            &MsgType::Err(err) => serializer.serialize_u8(err as u8),
        }
    }
}

struct MsgTypeVisitor;

impl<'de> Visitor<'de> for MsgTypeVisitor {
    type Value = MsgType;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid 8-bit CQC message type")
    }

    #[inline]
    fn visit_u8<E>(self, value: u8) -> Result<MsgType, E>
    where
        E: de::Error,
    {
        let msg_type = match MsgType::get_msg_type(value) {
            Some(msg_type) => msg_type,
            None => return Err(E::custom(format!("Invalid CQC message type: {}", value))),
        };

        Ok(msg_type)
    }
}

impl<'de> Deserialize<'de> for MsgType {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<MsgType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(MsgTypeVisitor)
    }
}

/// # CQC Command Header
///
/// A CQC Command Header identifies the specific instruction to execute, as
/// well as the qubit ID on which to perform this instructions.
///
/// A CQC Command Header MUST follow the CQC Header for the following messages:
///
///  - Execute a command list (msg_type=1).
///  - Start executing command list repeatedly (msg_type=2).
///  - Get creation time of qubit (msg_type=8).
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |            qubit_id           |     instr     |    options    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// Field     Length     Meaning
/// -----     ------     -------
/// qubit_id  2 bytes    Qubit ID to perform the operation on.
/// instr     1 byte     Instruction to perform.
/// options   1 byte     Options when executing the command.
/// ```
///
/// ## Possible Instruction Types
///
/// ```text
/// Type     Meaning
/// ----     -------
///  0       Identity (do nothing, wait one step).
///  1       Ask for a new qubit.
///  2       Measure qubit.
///  3       Measure qubit in-place.
///  4       Reset qubit to |0>.
///  5       Send qubit to another node.
///  6       Ask to receive qubit.
///  7       Create EPR pair with the specified node.
///  8       Receive EPR pair.
///
///  10      Pauli X.
///  11      Pauli Z.
///  12      Pauli Y.
///  13      T Gate.
///  14      Rotation over angle around X in pi/256 increments.
///  15      Rotation over angle around Y in pi/256 increments.
///  16      Rotation over angle around Z in pi/256 increments.
///  17      Hadamard Gate.
///  18      K Gate - taking computational to Y eigenbasis.
///
///  20      CNOT Gate with this as control.
///  21      CPHASE Gate with this as control.
/// ```
///
/// A CQC Xtra Header MUST follow the CQC Command Header for the following
/// instructions:
///
///  - Send qubit to another node (instr=5).
///  - Rotations (instr=14-16).
///  - Two qubit gates (instr=20,21).
///
/// ## Command options
///
/// Command options are set as bit flags.
///
/// ```text
/// Flag     Meaning
/// ----     -------
/// 0x01     Send a notification when command completes.
/// 0x02     On if there are actions to execute when done.
/// 0x04     Block until command is done.
/// 0x08     Execute command after done.
/// ```
///
/// ## Notify
///
/// If the notify option bit is set, each of these commands return a CQC
/// message indicating that execution has completed (msg_type=4). Some commands
/// also return additional messages, as described below:
///
/// - New qubit (instr=1): Returns an OK (msg_type=10) response followed by a
///                        notify header containing the qubit ID.
/// - Measurement (instr=2,3): Returns a measurement outcome message
///                            (msg_type=7) followed by a notify header
///                            containing the measurement outcome.
/// - Receive (instr=6): Returns a receive type message (msg_type=5) followed
///                      by a notify header containing the qubit ID.
/// - EPR (instr=7,8): Returns a response indicating EPR creation (msg_type=6)
///                    followed by a entanglement information header.
///

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CmdHdr {
    pub qubit_id: u16,
    pub instr: Cmd,
    pub options: u8,
}

pub const CMD_HDR_LENGTH: u32 = 4;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cmd {
    I = 0,              // Identity (do nothing, wait one step).
    New = 1,            // Ask for a new qubit.
    Measure = 2,        // Measure qubit.
    MeasureInplace = 3, // Measure qubit in-place.
    Reset = 4,          // Reset qubit to |0>.
    Send = 5,           // Send qubit to another node.
    Recv = 6,           // Ask to receive qubit.
    Epr = 7,            // Create EPR pair with the specified node.
    EprRecv = 8,        // Receive EPR pair.

    X = 10,    // Pauli X.
    Z = 11,    // Pauli Z.
    Y = 12,    // Pauli Y.
    T = 13,    // T Gate.
    RotX = 14, // Rotation over angle around X in pi/256 increments.
    RotY = 15, // Rotation over angle around Y in pi/256 increments.
    RotZ = 16, // Rotation over angle around Z in pi/256 increments.
    H = 17,    // Hadamard Gate.
    K = 18,    // K Gate - taking computational to Y eigenbasis.

    Cnot = 20,   // CNOT Gate with this as control.
    Cphase = 21, // CPHASE Gate with this as control.
}

impl Cmd {
    /// Convert an 8-bit value to a command type.  Returns `None` if the value
    /// does not correspond to a valid command type.
    #[inline]
    pub fn get_cmd(value: u8) -> Option<Cmd> {
        let command = match value {
            0 => Cmd::I,
            1 => Cmd::New,
            2 => Cmd::Measure,
            3 => Cmd::MeasureInplace,
            4 => Cmd::Reset,
            5 => Cmd::Send,
            6 => Cmd::Recv,
            7 => Cmd::Epr,
            8 => Cmd::EprRecv,

            10 => Cmd::X,
            11 => Cmd::Z,
            12 => Cmd::Y,
            13 => Cmd::T,
            14 => Cmd::RotX,
            15 => Cmd::RotY,
            16 => Cmd::RotZ,
            17 => Cmd::H,
            18 => Cmd::K,

            20 => Cmd::Cnot,
            21 => Cmd::Cphase,

            _ => return None,
        };

        Some(command)
    }
}

impl Serialize for Cmd {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

struct CmdVisitor;

impl<'de> Visitor<'de> for CmdVisitor {
    type Value = Cmd;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid 8-bit CQC isntruction type")
    }

    #[inline]
    fn visit_u8<E>(self, value: u8) -> Result<Cmd, E>
    where
        E: de::Error,
    {
        let instr = match Cmd::get_cmd(value) {
            Some(cmd) => cmd,
            None => {
                return Err(E::custom(format!(
                    "Invalid CQC instruction type: {}",
                    value
                )))
            }
        };

        Ok(instr)
    }
}

impl<'de> Deserialize<'de> for Cmd {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Cmd, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(CmdVisitor)
    }
}

pub const CMD_OPT_NOTIFY: u8 = 0x01; // Send a notification when command completes.
pub const CMD_OPT_ACTION: u8 = 0x02; // On if there are actions to execute when done.
pub const CMD_OPT_BLOCK: u8 = 0x04; // Block until command is done.
pub const CMD_OPT_IFTHEN: u8 = 0x08; // Execute command after done.

/// # CQC Xtra Header
///
/// Additional header containing further information for certain commands.
///
/// A CQC Xtra Header is required to follow the CQC Command Header for the
/// following instructions:
///
///  - Send qubit to another node (instr=5).
///  - Rotations (instr=14-16).
///  - Two qubit gates (instr=20,21).
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         xtra_qubit_id         |         remote_app_id         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          remote_node                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          cmd_length                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          remote_port          |     steps     |     align     |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// Field          Length     Meaning
/// -----          ------     -------
/// xtra_qubit_id  2 bytes    ID of the target qubit in a 2 qubit
///                           controlled gate.
/// remote_app_id  2 bytes    Remote Application ID.
/// remote_node    4 bytes    IP of the remote node (IPv4).
/// cmd_length     4 bytes    Length of the additional commands to execute upon
///                           completion.
/// remote_port    2 bytes    Port of the remode node for sending classical
///                           control info.
/// steps          1 byte     Angle step of rotation OR number of repetitions
///                           for a repeat command.
/// align          1 byte     4 byte alignment.
/// ```

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct XtraHdr {
    pub xtra_qubit_id: u16,
    pub remote_app_id: u16,
    pub remote_node: u32,
    pub cmd_length: u32,
    pub remote_port: u16,
    pub steps: u8,
    pub align: u8,
}

pub const XTRA_HDR_LENGTH: u32 = 16;

/// # CQC Notify Header
///
/// In some cases, the CQC Backend will return notifications to the client that
/// require additional information.  For example, where a qubit was received
/// from, the lifetime of a qubit, the measurement outcome etc.
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |            qubit_id           |         remote_app_id         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          remote_node                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           timestamp                           |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          remote_port          |    outcome    |     align     |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// Field          Length     Meaning
/// -----          ------     -------
/// qubit_id       2 bytes    ID of the received qubit.
/// remote_app_id  2 bytes    Remote application ID.
/// remote_node    4 bytes    IP of the remote node (IPv4).
/// timestamp      8 bytes    Time of creation.
/// remote_port    2 bytes    Port of the remote node for sending classical
///                           control info.
/// outcome        1 byte     Measurement outcome.
/// align          1 byte     4 byte alignment.
/// ```

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NotifyHdr {
    pub qubit_id: u16,
    pub remote_ap_id: u16,
    pub remote_node: u32,
    pub timestamp: u64,
    pub remote_port: u16,
    pub outcome: u8,
    pub align: u8,
}

pub const NOTIFY_HDR_LENGTH: u32 = 20;

/// # CQC Entanglement Information Header
///
/// When an EPR-pair is created the CQC Backend will return information about
/// the entanglement which can be used in a entanglement management protocol.
/// The entanglement information header contains information about the parties
/// that share the EPR-pair, the time of creation, how good the entanglement is
/// (goodness).  Furthermore, the entanglement information header contain a
/// entanglement ID (id_AB) which can be used to keep track of the entanglement
/// in the network.  The entanglement ID is incremented with respect to the
/// pair of nodes and who initialized the entanglement (DF).  For this reason
/// the entanglement ID together with the nodes and the directionality flag
/// gives a unique way to identify the entanglement in the network.
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             node_A                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |             port_A            |            app_id_A           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             node_B                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |             port_B            |            app_id_B           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             id_AB                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           timestamp                           |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                              ToG                              |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |            goodness           |       DF      |     align     |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// Field      Length     Meaning
/// -----      ------     -------
/// node_A     4 bytes    IP of this node.
/// port_A     2 bytes    Port of this node.
/// app_id_A   2 bytes    App ID of this node.
/// node_B     4 bytes    IP of other node.
/// port_B     2 bytes    Port of other node.
/// app_id_B   2 bytes    App ID of other node.
/// id_AB      4 bytes    Entanglement ID.
/// timestamp  8 bytes    Time of creation.
/// ToG        8 bytes    Time of goodness.
/// goodness   2 bytes    Goodness (estimate of the fidelity of state).
/// DF         1 byte     Directionality flag (0=Mid, 1=node_A, 2=node_B).
/// align      1 byte     4 byte alignment.
/// ```

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EntInfoHdr {
    pub node_a: u32,
    pub port_a: u16,
    pub app_id_a: u16,
    pub node_b: u32,
    pub port_b: u16,
    pub app_id_b: u16,
    pub id_ab: u32,
    pub timestamp: u64,
    pub tog: u64,
    pub goodness: u16,
    pub df: u8,
    pub align: u8,
}

pub const ENT_INFO_HDR_LENGTH: u32 = 40;

#[cfg(test)]
mod tests {
    extern crate bincode;

    use self::bincode::serialize;
    use super::*;

    #[test]
    fn cqc_hdr_ser_size() {
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: MsgType::Tp(Tp::Hello),
            app_id: 0,
            length: 0,
        };
        assert_eq!(serialize(&cqc_hdr).unwrap().len() as u32, CQC_HDR_LENGTH);
    }

    #[test]
    fn cmd_hdr_ser_size() {
        let cmd_hdr = CmdHdr {
            qubit_id: 0,
            instr: Cmd::I,
            options: 0,
        };
        assert_eq!(serialize(&cmd_hdr).unwrap().len() as u32, CMD_HDR_LENGTH);
    }

    #[test]
    fn xtra_hdr_ser_size() {
        let xtra_hdr = XtraHdr {
            xtra_qubit_id: 0,
            remote_app_id: 0,
            remote_node: 0,
            cmd_length: 0,
            remote_port: 0,
            steps: 0,
            align: 0,
        };
        assert_eq!(serialize(&xtra_hdr).unwrap().len() as u32, XTRA_HDR_LENGTH);
    }

    #[test]
    fn notify_hdr_ser_size() {
        let notify_hdr = NotifyHdr {
            qubit_id: 0,
            remote_ap_id: 0,
            remote_node: 0,
            timestamp: 0,
            remote_port: 0,
            outcome: 0,
            align: 0,
        };
        assert_eq!(
            serialize(&notify_hdr).unwrap().len() as u32,
            NOTIFY_HDR_LENGTH
        );
    }

    #[test]
    fn ent_info_hdr_ser_size() {
        let ent_info_hdr = EntInfoHdr {
            node_a: 0,
            port_a: 0,
            app_id_a: 0,
            node_b: 0,
            port_b: 0,
            app_id_b: 0,
            id_ab: 0,
            timestamp: 0,
            tog: 0,
            goodness: 0,
            df: 0,
            align: 0,
        };
        assert_eq!(
            serialize(&ent_info_hdr).unwrap().len() as u32,
            ENT_INFO_HDR_LENGTH
        );
    }
}
