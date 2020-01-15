//! # CQC Interface
//!
//! This module documents the [CQC Interface
//! specification](https://softwarequtech.github.io/CQC-Python/interface.html)
//! and defines the necessary constants and header structures.
//!
//! # CQC Header
//!
//! Every CQC message begins with a CQC header.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! version   1 byte     CQC interface version.  Current version is 2.
//! type      1 byte     Message type.
//! app_id    2 bytes    Application ID.  Return messages will be tagged
//!                      appropriately.
//! length    4 bytes    Total length of the CQC instruction packet.
//! ```
//!
//! A CQC Command Header MUST follow the CQC Header for the following messages:
//!
//!  - Command
//!  - Factory
//!  - GetTime
//!
//! ## CQC Header Message Types
//!
//! The supported message types.  They are split into normal types (Tp) and
//! error types (Err).
//!
//! ```text
//! Type     Name     Meaning
//! ----     ----     -------
//!  0       Hello    Alive check.
//!  1       Command  Execute a command list.
//!  2       Factory  Start executing command list repeatedly.
//!  3       Expire   Qubit has expired.
//!  4       Done     Command execution done.
//!  5       Recv     Received qubit.
//!  6       EprOk    Created EPR pair.
//!  7       MeasOut  Measurement outcome.
//!  8       GetTime  Get creation time of qubit.
//!  9       InfTime  Get timing informaiton.
//!  10      NewOk    Created new qubit.
//!  11      Mix      Multiple header types will follow.
//!  12      If       Perform a conditional action.
//!
//!  20      General  General purpose error (no details).
//!  21      NoQubit  No more qubits available.
//!  22      Unsupp   Command sequence not supported.
//!  23      Timeout  Timeout.
//!  24      InUse    Qubit already in use.
//!  25      Unknown  Unknown qubit ID.
//! ```
//!
//! # CQC Command Header
//!
//! A CQC Command Header identifies the specific instruction to execute, as
//! well as the qubit ID on which to perform this instructions.
//!
//! A CQC Command Header MUST follow the CQC Header for the following messages:
//!
//!  - Command
//!  - Factory
//!  - GetTime
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! qubit_id  2 bytes    Qubit ID to perform the operation on.
//! instr     1 byte     Instruction to perform.
//! options   1 byte     Options when executing the command.
//! ```
//!
//! ## Notify
//!
//! If the notify option bit is set, each of these commands return a CQC
//! message Done indicating that execution has completed. Some commands also
//! return additional messages, as described below:
//!
//! - New: Returns a NewOk reply followed by an Extra Qubit header with the
//!        qubit ID.
//! - Measure(InPlace): Returns a MeasOut message followed by a Measurement
//!                     Outcome header containing the measurement outcome.
//! - Recv: Returns a Recv reply followed by an Extra Qubit header with the
//!         qubit ID.
//! - Epr(Recv): Returns an EprOk reply by an Extra Qubit header and an
//!              Entanglement Information header.
//!
//! ## CQC Command Header Instruction Types
//!
//! The supported CQC instructions.
//!
//! ```text
//! Type     Name            Meaning
//! ----     ----            -------
//!  0       I               Identity (do nothing, wait one step).
//!  1       New             Ask for a new qubit.
//!  2       Measure         Measure qubit.
//!  3       MeasureInPlace  Measure qubit in-place.
//!  4       Reset           Reset qubit to |0>.
//!  5       Send            Send qubit to another node.
//!  6       Recv            Ask to receive qubit.
//!  7       Epr             Create EPR pair with the specified node.
//!  8       EprRecv         Receive EPR pair.
//!
//!  10      X               Pauli X.
//!  11      Z               Pauli Z.
//!  12      Y               Pauli Y.
//!  13      T               T Gate.
//!  14      RotX            Rotation over angle around X in pi/256 increments.
//!  15      RotY            Rotation over angle around Y in pi/256 increments.
//!  16      RotZ            Rotation over angle around Z in pi/256 increments.
//!  17      H               Hadamard Gate.
//!  18      K               K Gate - taking computational to Y eigenbasis.
//!
//!  20      Cnot            CNOT Gate with this as control.
//!  21      Cphase          CPHASE Gate with this as control.
//!
//!  22      Allocate        Allocate a number of qubits.
//!  23      Release         Release a qubit.
//! ```
//!
//! ## CQC Command Header options
//!
//! Command options are set as bit flags.
//!
//! ```text
//! Flag     Name     Meaning
//! ----     ----     -------
//! 0x01     Notify   Send a notification when command completes.
//! 0x02     Action   On if there are actions to execute when done.
//! 0x04     Block    Block until command is done.
//! 0x08     IfThen   Execute command after done.
//! ```
//!
//! # CQC Assign Header
//!
//! Additional header used to store a measurement outcome in the backend and
//! assign it a reference ID.  Every measurement command (CQC_CMD_MEASURE or
//! CQC_CMD_MEASURE_INPLACE) is followed by a CQC Assign Header.  The value can
//! be retrieved by future instructions by refering to this ID.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! ref_id    4 bytes    Reference ID for the measurement` value.
//! ``
//!
//! # CQC Rotation Header
//!
//! Additional header used to define the rotation angle of a rotation gate.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! step      1 byte     Angle step of rotation (increments of 1/256).
//! ```
//!
//! # CQC Extra Qubit Header
//!
//! Additional header used to send the qubit_id of a secondary qubit for two
//! qubit gates.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! qubit_id  2 bytes    ID of the target qubit.
//! ```
//!
//! # CQC Communication Header
//!
//! Additional header used to send to which node to send information to. Used
//! in send and EPR commands.
//!
//! ```text
//! Field          Length     Meaning
//! -----          ------     -------
//! remote_app_id  2 bytes    Remote application ID.
//! remote_port    2 bytes    Port of the remote node for sending classical
//!                           control info.
//! remote_node    4 bytes    IP of the remote node (IPv4).
//! ```
//!
//! # CQC Factory Header
//!
//! Additional header used to send factory information. Factory commands are
//! used to tell the backend to do the following command or a sequence of
//! commands multiple times.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! num_iter  1 byte     Number of iterations to do the sequence.
//! options   1 byte     Options when executing the factory.
//! ```
//!
//! ## CQC Factory Header options
//!
//! Factory options are set as bit flags.
//!
//! ```text
//! Flag     Name     Meaning
//! ----     ----     -------
//! 0x01     Notify   Send a notification when command completes.
//! 0x04     Block    Block until factory is done.
//! ```
//!
//! # CQC Measurement Outcome Header
//!
//! Additional header used to send the outcome of a measurement.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! meas_out  1 byte     Measurement outcome.
//! ```
//!
//! # CQC Time Info Header
//!
//! Additional header used to send time information in response to the GetTime
//! command.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! datetime  8 bytes    Time of creation.
//! ```
//!
//! # CQC Entanglement Information Header
//!
//! When an EPR-pair is created the CQC Backend will return information about
//! the entanglement which can be used in a entanglement management protocol.
//! The entanglement information header contains information about the parties
//! that share the EPR-pair, the time of creation, how good the entanglement is
//! (goodness).  Furthermore, the entanglement information header contain a
//! entanglement ID (id_AB) which can be used to keep track of the entanglement
//! in the network.  The entanglement ID is incremented with respect to the
//! pair of nodes and who initialized the entanglement (DF).  For this reason
//! the entanglement ID together with the nodes and the directionality flag
//! gives a unique way to identify the entanglement in the network.
//!
//! ```text
//! Field      Length     Meaning
//! -----      ------     -------
//! node_A     4 bytes    IP of this node.
//! port_A     2 bytes    Port of this node.
//! app_id_A   2 bytes    App ID of this node.
//! node_B     4 bytes    IP of other node.
//! port_B     2 bytes    Port of other node.
//! app_id_B   2 bytes    App ID of other node.
//! id_AB      4 bytes    Entanglement ID.
//! timestamp  8 bytes    Time of creation.
//! ToG        8 bytes    Time of goodness.
//! goodness   2 bytes    Goodness (estimate of the fidelity of state).
//! DF         1 byte     Directionality flag (0=Mid, 1=node_A, 2=node_B).
//! align      1 byte     4 byte alignment.
//! ```
//!
//! # CQC Type Header
//!
//! A top-level CQC header of type Mix may be followed by multiple other header
//! types.  Each new header is announced by the CQC type header.
//!
//! ```text
//! Field     Length     Meaning
//! -----     ------     -------
//! type      1 byte     Type of next header (except Mix).
//! length    4 bytes    Number of bytes until the next type header.
//! ```
//!
//! # CQC If Header
//!
//! The If header can only be used inside programs of type Mix.  Execute the
//! following command only if the specified condition is true.
//!
//! ```text
//! Field          Length     Meaning
//! -----          ------     -------
//! left_operand   4 bytes    Refernce ID of the first operand.
//! operator       1 byte     Comparison operator.
//! right_type     1 byte     Type of second operand.
//! right_operand  4 bytes    Reference ID or value of second operand.
//! length         4 bytes    Length in bytes of following command.
//! ```
//!
//! ## CQC If Header Operator Types
//!
//! ```text
//! Type     Name     Meaning
//! ----     ----     -------
//!  0       Eq       Compare for equality.
//!  1       InEq     Compare for inequality.
//! ```
//!
//! ## CQC If Header Right Operand Types
//!
//! ```text
//! Type     Name     Meaning
//! ----     ----     -------
//!  0       Value    Right operand holds raw value.
//!  1       RefId    Right operand holds reference ID.
//! ```

extern crate serde;

use self::serde::de;
use std::fmt;

use self::serde::de::Visitor;
use self::serde::{Deserialize, Deserializer, Serialize, Serializer};

#[macro_use]
mod macros;

/// # CQC Version
///
/// The current supported versions are: 2.
/// The currently unsupported versions are: 0, 1.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Version {
    V2 = 2,
}

impl Version {
    /// Convert an 8-bit value to a version value.  Returns `None` if the value
    /// does not correspond to a currently supported version.
    #[inline]
    pub fn get(value: u8) -> Option<Version> {
        let version = match value {
            2 => Version::V2,
            _ => return None,
        };

        Some(version)
    }
}

serde_enum_u8!(Version, VersionVisitor, "CQC version");

/// # CQC Header
///
/// Every CQC message begins with a CQC header.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// version   1 byte     CQC interface version.  Current version is 2.
/// type      1 byte     Message type.
/// app_id    2 bytes    Application ID.  Return messages will be tagged
///                      appropriately.
/// length    4 bytes    Total length of the CQC instruction packet.
/// ```
///
/// A CQC Command Header MUST follow the CQC Header for the following messages:
///
///  - Command
///  - Factory
///  - GetTime
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CqcHdr {
    pub version: Version,
    pub msg_type: MsgType,
    pub app_id: u16,
    pub length: u32,
}

def_len!(CqcHdr, 8);

/// # CQC Header Message Types
///
/// The supported message types.  They are split into normal types (Tp) and
/// error types (Err).
///
/// ```text
/// Type     Name     Meaning
/// ----     ----     -------
///  0       Hello    Alive check.
///  1       Command  Execute a command list.
///  2       Factory  Start executing command list repeatedly.
///  3       Expire   Qubit has expired.
///  4       Done     Command execution done.
///  5       Recv     Received qubit.
///  6       EprOk    Created EPR pair.
///  7       MeasOut  Measurement outcome.
///  8       GetTime  Get creation time of qubit.
///  9       InfTime  Get timing informaiton.
///  10      NewOk    Created new qubit.
///  11      Mix      Multiple header types will follow.
///  12      If       Perform a conditional action.
///
///  20      General  General purpose error (no details).
///  21      NoQubit  No more qubits available.
///  22      Unsupp   Command sequence not supported.
///  23      Timeout  Timeout.
///  24      InUse    Qubit already in use.
///  25      Unknown  Unknown qubit ID.
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MsgType {
    Tp(Tp),
    Err(Err),
}

impl From<MsgType> for u8 {
    fn from(msg_type: MsgType) -> Self {
        match msg_type {
            MsgType::Tp(val) => val as u8,
            MsgType::Err(val) => val as u8,
        }
    }
}

macro_rules! def_is_tp {
    ($tp: pat, $name: ident) => {
        #[inline]
        pub fn $name(&self) -> bool {
            match self {
                &MsgType::Tp($tp) => true,
                _ => false,
            }
        }
    }
}

macro_rules! def_is_err {
    ($tp: pat, $name: ident) => {
        #[inline]
        pub fn $name(&self) -> bool {
            match self {
                &MsgType::Err($tp) => true,
                _ => false,
            }
        }
    }
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

    def_is_tp!(Tp::Hello, is_hello);
    def_is_tp!(Tp::Command, is_command);
    def_is_tp!(Tp::Factory, is_factory);
    def_is_tp!(Tp::Expire, is_expire);
    def_is_tp!(Tp::Done, is_done);
    def_is_tp!(Tp::Recv, is_recv);
    def_is_tp!(Tp::EprOk, is_epr_ok);
    def_is_tp!(Tp::MeasOut, is_measout);
    def_is_tp!(Tp::GetTime, is_get_time);
    def_is_tp!(Tp::InfTime, is_inf_time);
    def_is_tp!(Tp::NewOk, is_new_ok);
    def_is_tp!(Tp::Mix, is_mix);
    def_is_tp!(Tp::If, is_if);

    def_is_err!(Err::General, is_err_general);
    def_is_err!(Err::NoQubit, is_err_noqubit);
    def_is_err!(Err::Unsupp, is_err_unsupp);
    def_is_err!(Err::Timeout, is_err_timeout);
    def_is_err!(Err::InUse, is_err_inuse);
    def_is_err!(Err::Unknown, is_err_unknown);

    /// Convert an 8-bit value to a message type.  Returns `None` if the value
    /// does not correspond to a valid message type.
    #[inline]
    pub fn get(value: u8) -> Option<MsgType> {
        let msg_type = if value <= Tp::If as u8 {
            MsgType::Tp(Tp::get(value).unwrap())
        } else if value >= Err::General as u8 && value <= Err::Unknown as u8 {
            MsgType::Err(Err::get(value).unwrap())
        } else {
            return None;
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

deserialize_enum_u8!(MsgType, MsgTypeVisitor, "CQC message type");

/// # CQC Header Normal Message Types
///
/// The supported normal message types.
///
/// ```text
/// Type     Name     Meaning
/// ----     ----     -------
///  0       Hello    Alive check.
///  1       Command  Execute a command list.
///  2       Factory  Start executing command list repeatedly.
///  3       Expire   Qubit has expired.
///  4       Done     Command execution done.
///  5       Recv     Received qubit.
///  6       EprOk    Created EPR pair.
///  7       MeasOut  Measurement outcome.
///  8       GetTime  Get creation time of qubit.
///  9       InfTime  Get timing informaiton.
///  10      NewOk    Created new qubit.
///  11      Mix      Multiple header types will follow.
///  12      If       Perform a conditional action.
/// ```
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
    MeasOut = 7, // Measurement outcome.
    GetTime = 8, // Get creation time of qubit.
    InfTime = 9, // Inform about time.
    NewOk = 10,  // Created new qubit.
    Mix = 11,    // Multiple header types will follow.
    If = 12,     // Perform a conditional action.
}

impl Tp {
    /// Convert an 8-bit value to a normal message type.  Returns `None` if the
    /// value does not correspond to a valid normal message type.
    #[inline]
    pub fn get(value: u8) -> Option<Tp> {
        let msg_type = match value {
            0 => Tp::Hello,
            1 => Tp::Command,
            2 => Tp::Factory,
            3 => Tp::Expire,
            4 => Tp::Done,
            5 => Tp::Recv,
            6 => Tp::EprOk,
            7 => Tp::MeasOut,
            8 => Tp::GetTime,
            9 => Tp::InfTime,
            10 => Tp::NewOk,
            11 => Tp::Mix,
            12 => Tp::If,

            _ => return None,
        };

        Some(msg_type)
    }
}

serde_enum_u8!(Tp, TpVisitor, "CQC normal message type");

/// # CQC Header Error Message Types
///
/// The supported error message types.
///
/// ```text
/// Type     Name     Meaning
/// ----     ----     -------
///  20      General  General purpose error (no details).
///  21      NoQubit  No more qubits available.
///  22      Unsupp   Command sequence not supported.
///  23      Timeout  Timeout.
///  24      InUse    Qubit already in use.
///  25      Unknown  Unknown qubit ID.
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Err {
    General = 20, // General purpose error (no details).
    NoQubit = 21, // No more qubits available.
    Unsupp = 22,  // Command sequence not supported.
    Timeout = 23, // Timeout.
    InUse = 24,   // Qubit already in use.
    Unknown = 25, // Unknown qubit ID
}

impl Err {
    /// Convert an 8-bit value to an error message type.  Returns `None` if the
    /// value does not correspond to a valid normal message type.
    #[inline]
    pub fn get(value: u8) -> Option<Err> {
        let msg_type = match value {
            20 => Err::General,
            21 => Err::NoQubit,
            22 => Err::Unsupp,
            23 => Err::Timeout,
            24 => Err::InUse,
            25 => Err::Unknown,

            _ => return None,
        };

        Some(msg_type)
    }
}

serde_enum_u8!(Err, ErrVisitor, "CQC error message type");

/// # CQC Command Header
///
/// A CQC Command Header identifies the specific instruction to execute, as
/// well as the qubit ID on which to perform this instructions.
///
/// A CQC Command Header MUST follow the CQC Header for the following messages:
///
///  - Command
///  - Factory
///  - GetTime
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// qubit_id  2 bytes    Qubit ID to perform the operation on.
/// instr     1 byte     Instruction to perform.
/// options   1 byte     Options when executing the command.
/// ```
///
/// ## Notify
///
/// If the notify option bit is set, each of these commands return a CQC
/// message Done indicating that execution has completed. Some commands also
/// return additional messages, as described below:
///
/// - New: Returns a NewOk reply followed by an Extra Qubit header with the
///        qubit ID.
/// - Measure(InPlace): Returns a MeasOut message followed by a Measurement
///                     Outcome header containing the measurement outcome.
/// - Recv: Returns a Recv reply followed by an Extra Qubit header with the
///         qubit ID.
/// - Epr(Recv): Returns an EprOk reply by an Extra Qubit header and an
///              Entanglement Information header.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CmdHdr {
    pub qubit_id: u16,
    pub instr: Cmd,
    pub options: CmdOpt,
}

def_len!(CmdHdr, 4);

/// # CQC Command Header Instruction Types
///
/// The supported CQC instructions.
///
/// ```text
/// Type     Name            Meaning
/// ----     ----            -------
///  0       I               Identity (do nothing, wait one step).
///  1       New             Ask for a new qubit.
///  2       Measure         Measure qubit.
///  3       MeasureInPlace  Measure qubit in-place.
///  4       Reset           Reset qubit to |0>.
///  5       Send            Send qubit to another node.
///  6       Recv            Ask to receive qubit.
///  7       Epr             Create EPR pair with the specified node.
///  8       EprRecv         Receive EPR pair.
///
///  10      X               Pauli X.
///  11      Z               Pauli Z.
///  12      Y               Pauli Y.
///  13      T               T Gate.
///  14      RotX            Rotation over angle around X in pi/256 increments.
///  15      RotY            Rotation over angle around Y in pi/256 increments.
///  16      RotZ            Rotation over angle around Z in pi/256 increments.
///  17      H               Hadamard Gate.
///  18      K               K Gate - taking computational to Y eigenbasis.
///
///  20      Cnot            CNOT Gate with this as control.
///  21      Cphase          CPHASE Gate with this as control.
///
///  22      Allocate        Allocate a number of qubits.
///  23      Release         Release a qubit.
/// ```
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

    Allocate = 22, // Allocate a number of qubits.
    Release = 23,  // Release a qubit.
}

impl Cmd {
    /// Convert an 8-bit value to a command type.  Returns `None` if the value
    /// does not correspond to a valid command type.
    #[inline]
    pub fn get(value: u8) -> Option<Cmd> {
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

            22 => Cmd::Allocate,
            23 => Cmd::Release,

            _ => return None,
        };

        Some(command)
    }
}

serde_enum_u8!(Cmd, CmdVisitor, "CQC instruction type");

bitflags! {
    /// # CQC Command Header options
    ///
    /// Command options are set as bit flags.
    ///
    /// ```text
    /// Flag     Name     Meaning
    /// ----     ----     -------
    /// 0x01     Notify   Send a notification when command completes.
    /// 0x02     Action   On if there are actions to execute when done.
    /// 0x04     Block    Block until command is done.
    /// 0x08     IfThen   Execute command after done.
    /// ```
    pub struct CmdOpt: u8 {
        const NOTIFY = 0x01;
        const ACTION = 0x02;
        const BLOCK = 0x04;
        const IFTHEN = 0x08;
    }
}

impl CmdOpt {
    def_set_flag!(CmdOpt, NOTIFY, set_notify);
    def_set_flag!(CmdOpt, ACTION, set_action);
    def_set_flag!(CmdOpt, BLOCK, set_block);
    def_set_flag!(CmdOpt, IFTHEN, set_ifthen);

    def_get_flag!(CmdOpt, NOTIFY, get_notify);
    def_get_flag!(CmdOpt, ACTION, get_action);
    def_get_flag!(CmdOpt, BLOCK, get_block);
    def_get_flag!(CmdOpt, IFTHEN, get_ifthen);
}

serde_option_u8!(CmdOpt, CmdOptVisitor, "command");

/// # CQC Assign Header
///
/// Additional header used to store a measurement outcome in the backend and
/// assign it a reference ID.  Every measurement command (CQC_CMD_MEASURE or
/// CQC_CMD_MEASURE_INPLACE) is followed by a CQC Assign Header.  The value can
/// be retrieved by future instructions by refering to this ID.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// ref_id    4 bytes    Reference ID for the measurement` value.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssignHdr {
    pub ref_id: u32,
}

def_len!(AssignHdr, 4);

/// # CQC Rotation Header
///
/// Additional header used to define the rotation angle of a rotation gate.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// step      1 byte     Angle step of rotation (increments of 1/256).
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RotHdr {
    pub step: u8,
}

def_len!(RotHdr, 1);

/// # CQC Extra Qubit Header
///
/// Additional header used to send the qubit_id of a secondary qubit for two
/// qubit gates.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// qubit_id  2 bytes    ID of the target qubit.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QubitHdr {
    pub qubit_id: u16,
}

def_len!(QubitHdr, 2);

/// # CQC Communication Header
///
/// Additional header used to send to which node to send information to. Used
/// in send and EPR commands.
///
///
/// ```text
/// Field          Length     Meaning
/// -----          ------     -------
/// remote_app_id  2 bytes    Remote application ID.
/// remote_port    2 bytes    Port of the remote node for sending classical
///                           control info.
/// remote_node    4 bytes    IP of the remote node (IPv4).
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommHdr {
    pub remote_app_id: u16,
    pub remote_port: u16,
    pub remote_node: u32,
}

def_len!(CommHdr, 8);

/// # CQC Factory Header
///
/// Additional header used to send factory information. Factory commands are
/// used to tell the backend to do the following command or a sequence of
/// commands multiple times.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// num_iter  1 byte     Number of iterations to do the sequence.
/// options   1 byte     Options when executing the factory.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FactoryHdr {
    pub num_iter: u8,
    pub options: FactoryOpt,
}

def_len!(FactoryHdr, 2);

bitflags! {
    /// # CQC Factory Header options
    ///
    /// Factory options are set as bit flags.
    ///
    /// ```text
    /// Flag     Name     Meaning
    /// ----     ----     -------
    /// 0x01     Notify   Send a notification when command completes.
    /// 0x04     Block    Block until factory is done.
    /// ```
    pub struct FactoryOpt: u8 {
        const NOTIFY = 0x01;
        const BLOCK = 0x04;
    }
}

impl FactoryOpt {
    def_set_flag!(FactoryOpt, NOTIFY, set_notify);
    def_set_flag!(FactoryOpt, BLOCK, set_block);

    def_get_flag!(FactoryOpt, NOTIFY, get_notify);
    def_get_flag!(FactoryOpt, BLOCK, get_block);
}

serde_option_u8!(FactoryOpt, FactoryOptVisitor, "factory");

/// # CQC Measurement Outcome Header
///
/// Additional header used to send the outcome of a measurement.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// meas_out  1 byte     Measurement outcome.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MeasOutHdr {
    pub meas_out: MeasOut,
}

def_len!(MeasOutHdr, 1);

/// # CQC Measurement outcome
///
/// There are only two possible outcome values: 0 or 1.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MeasOut {
    Zero = 0,
    One = 1,
}

impl MeasOut {
    /// Convert an 8-bit value to a measurement outcome value.  Returns `None`
    /// if the value does not correspond to a valid outcome.
    #[inline]
    pub fn get(value: u8) -> Option<MeasOut> {
        let meas_out = match value {
            0 => MeasOut::Zero,
            1 => MeasOut::One,
            _ => return None,
        };

        Some(meas_out)
    }
}

serde_enum_u8!(MeasOut, MeasOutVisitor, "Measurement Outcome");

/// # CQC Time Info Header
///
/// Additional header used to send time information in response to the GetTime
/// command.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// datetime  8 bytes    Time of creation.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TimeInfoHdr {
    pub datetime: u64,
}

def_len!(TimeInfoHdr, 8);

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

def_len!(EntInfoHdr, 40);

/// # CQC Type Header
///
/// A top-level CQC header of type Mix may be followed by multiple other header
/// types.  Each new header is announced by the CQC type header.
///
/// ```text
/// Field     Length     Meaning
/// -----     ------     -------
/// type      1 byte     Type of next header (except Mix).
/// length    4 bytes    Number of bytes until the next type header.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TypeHdr {
    pub hdr_type: Tp,
    pub length: u32,
}

def_len!(TypeHdr, 5);

/// # CQC If Header
///
/// The If header can only be used inside programs of type Mix.  Execute the
/// following command only if the specified condition is true.
///
/// ```text
/// Field          Length     Meaning
/// -----          ------     -------
/// left_operand   4 bytes    Refernce ID of the first operand.
/// operator       1 byte     Comparison operator.
/// right_type     1 byte     Type of second operand.
/// right_operand  4 bytes    Reference ID or value of second operand.
/// length         4 bytes    Length in bytes of following command.
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IfHdr {
    pub left_op: u32,
    pub operator: CmpType,
    pub right_op_t: OpType,
    pub right_op: u32,
    pub length: u32,
}

def_len!(IfHdr, 14);

/// ## CQC If Header Operator Types
///
/// ```text
/// Type     Name     Meaning
/// ----     ----     -------
///  0       Eq       Compare for equality.
///  1       InEq     Compare for inequality.
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmpType {
    Eq = 0,
    InEq = 1,
}

impl CmpType {
    /// Convert an 8-bit value to a comparison operator type.  Returns `None`
    /// if the value does not correspond to a valid operator type.
    #[inline]
    pub fn get(value: u8) -> Option<CmpType> {
        let cmp_type = match value {
            0 => CmpType::Eq,
            1 => CmpType::InEq,
            _ => return None,
        };

        Some(cmp_type)
    }
}

serde_enum_u8!(CmpType, CmpTypeVisitor, "Comparison Operator Type");

/// ## CQC If Header Right Operand Types
///
/// ```text
/// Type     Name     Meaning
/// ----     ----     -------
///  0       Value    Right operand holds raw value.
///  1       RefId    Right operand holds reference ID.
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpType {
    Value = 0,
    RefId = 1,
}

impl OpType {
    /// Convert an 8-bit value to an operand type.  Returns `None` if the value
    /// does not correspond to a valid operand type.
    #[inline]
    pub fn get(value: u8) -> Option<OpType> {
        let op_type = match value {
            0 => OpType::Value,
            1 => OpType::RefId,
            _ => return None,
        };

        Some(op_type)
    }
}

serde_enum_u8!(OpType, OpTypeVisitor, "Operand Type");

// ----------------------------------------------------------------------------
// Tests.
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    extern crate bincode;

    use self::bincode::serialize;
    use super::*;

    #[test]
    fn cqc_hdr_ser_size() {
        let cqc_hdr = CqcHdr {
            version: Version::V2,
            msg_type: MsgType::Tp(Tp::Hello),
            app_id: 0,
            length: 0,
        };
        assert_eq!(serialize(&cqc_hdr).unwrap().len() as u32, cqc_hdr.len());
    }

    #[test]
    fn cmd_hdr_ser_size() {
        let cmd_hdr = CmdHdr {
            qubit_id: 0,
            instr: Cmd::I,
            options: CmdOpt::empty(),
        };
        assert_eq!(serialize(&cmd_hdr).unwrap().len() as u32, cmd_hdr.len());
    }

    #[test]
    fn assign_hdr_ser_size() {
        let assign_hdr = AssignHdr { ref_id: 0 };
        assert_eq!(
            serialize(&assign_hdr).unwrap().len() as u32,
            assign_hdr.len()
        );
    }

    #[test]
    fn rot_hdr_ser_size() {
        let rot_hdr = RotHdr { step: 0 };
        assert_eq!(serialize(&rot_hdr).unwrap().len() as u32, rot_hdr.len());
    }

    #[test]
    fn qubit_hdr_ser_size() {
        let qubit_hdr = QubitHdr { qubit_id: 0 };
        assert_eq!(
            serialize(&qubit_hdr).unwrap().len() as u32,
            qubit_hdr.len()
        );
    }

    #[test]
    fn comm_hdr_ser_size() {
        let comm_hdr = CommHdr {
            remote_app_id: 0,
            remote_node: 0,
            remote_port: 0,
        };
        assert_eq!(serialize(&comm_hdr).unwrap().len() as u32, comm_hdr.len());
    }

    #[test]
    fn factory_hdr_ser_size() {
        let factory_hdr = FactoryHdr {
            num_iter: 0,
            options: FactoryOpt::empty(),
        };
        assert_eq!(
            serialize(&factory_hdr).unwrap().len() as u32,
            factory_hdr.len()
        );
    }

    #[test]
    fn meas_out_hdr_ser_size() {
        let meas_out_hdr = MeasOutHdr {
            meas_out: MeasOut::Zero,
        };
        assert_eq!(
            serialize(&meas_out_hdr).unwrap().len() as u32,
            meas_out_hdr.len()
        );
    }

    #[test]
    fn time_info_hdr_ser_size() {
        let time_info_hdr = TimeInfoHdr { datetime: 0 };
        assert_eq!(
            serialize(&time_info_hdr).unwrap().len() as u32,
            time_info_hdr.len()
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
            ent_info_hdr.len()
        );
    }

    #[test]
    fn type_hdr_ser_size() {
        let type_hdr = TypeHdr {
            hdr_type: Tp::Hello,
            length: 0,
        };
        assert_eq!(serialize(&type_hdr).unwrap().len() as u32, type_hdr.len());
    }

    #[test]
    fn if_hdr_ser_size() {
        let if_hdr = IfHdr {
            left_op: 0,
            operator: CmpType::Eq,
            right_op_t: OpType::Value,
            right_op: 0,
            length: 0,
        };
        assert_eq!(serialize(&if_hdr).unwrap().len() as u32, if_hdr.len());
    }
}
