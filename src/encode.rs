//! # CQC Encoder
//!
//! This module provides the encoder for the CQC protocol.  The encoder does
//! not check for protocol correctness.

extern crate bincode;

use Request;

pub struct Encoder {
    config: bincode::Config,
}

impl Encoder {
    /// Create a big endian `Encoder`.
    pub fn new() -> Encoder {
        let mut config = bincode::config();
        config.big_endian();

        Encoder { config }
    }

    /// Encode a CQC request packet into buffer of bytes.  The return value is
    /// a the number of bytes written.
    ///
    /// If the provided buffer is not large enough to encode the request
    /// `encode_request` will panic.
    pub fn encode_request<'buf>(&self, request: &Request, buffer: &'buf mut [u8]) -> usize {
        let len = request.len() as usize;
        assert!(buffer.len() >= len);
        self.config
            .serialize_into(&mut buffer[..len], &request)
            .unwrap();

        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {ReqCmd, Request, XtraHdr};
    use hdr::*;

    macro_rules! get_byte_16 {
        ($value:expr, $byte:expr) => {
            ($value >> ((1 - $byte) * 8)) as u8
        };
    }

    macro_rules! get_byte_32 {
        ($value:expr, $byte:expr) => {
            ($value >> ((3 - $byte) * 8)) as u8
        };
    }

    // Set up constants.
    const APP_ID: u16 = 0x0A_0E;
    const QUBIT_ID: u16 = 0xBE_56;
    const EXTRA_QUBIT_ID: u16 = 0xFE_80;
    const REMOTE_APP_ID: u16 = 0x5E_3F;
    const REMOTE_NODE: u32 = 0xAE_04_E2_52;
    const REMOTE_PORT: u16 = 0x91_03;
    const STEP: u8 = 192;

    // Encode a request packet that only has a CQC header.
    #[test]
    fn cqc_hdr_encode() {
        let cqc_type = Tp::Hello;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = 0;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: None,
        };

        // Buffer to write into.
        let buf_len: usize = (CqcHdr::hdr_len() + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Big-endian
        let expected: Vec<u8> = vec![
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet that has a CMD header, but no XTRA header.
    #[test]
    fn cmd_hdr_encode() {
        let cqc_type = Tp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CmdHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::New;
        let mut options = CmdOpt::empty();
        options.set_notify().set_block();

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: XtraHdr::None,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CqcHdr::hdr_len() + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // CMD header
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            instr as u8,
            options.bits(),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and ROT headers.
    #[test]
    fn rot_hdr_encode() {
        let cqc_type = Tp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CmdHdr::hdr_len() + RotHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::RotX;
        let mut options = CmdOpt::empty();
        options.set_notify().set_block();

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        // The XTRA header.
        let xtra_hdr = RotHdr { step: STEP };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: XtraHdr::Rot(xtra_hdr),
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CqcHdr::hdr_len() + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // CMD header
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            instr as u8,
            options.bits(),
            // XTRA header
            STEP,
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and QUBIT headers.
    #[test]
    fn qubit_hdr_encode() {
        let cqc_type = Tp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CmdHdr::hdr_len() + QubitHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::Cnot;
        let mut options = CmdOpt::empty();
        options.set_notify().set_block();

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        // The XTRA header.
        let xtra_hdr = QubitHdr { qubit_id: EXTRA_QUBIT_ID };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: XtraHdr::Qubit(xtra_hdr),
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CqcHdr::hdr_len() + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // CMD header
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            instr as u8,
            options.bits(),
            // XTRA header
            get_byte_16!(EXTRA_QUBIT_ID, 0),
            get_byte_16!(EXTRA_QUBIT_ID, 1),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and COMM headers.
    #[test]
    fn comm_hdr_encode() {
        let cqc_type = Tp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CmdHdr::hdr_len() + CommHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::Send;
        let mut options = CmdOpt::empty();
        options.set_notify().set_block();

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        // The XTRA header.
        let xtra_hdr = CommHdr {
            remote_app_id: REMOTE_APP_ID,
            remote_node: REMOTE_NODE,
            remote_port: REMOTE_PORT,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: XtraHdr::Comm(xtra_hdr),
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CqcHdr::hdr_len() + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // CMD header
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            instr as u8,
            options.bits(),
            // XTRA header
            get_byte_16!(REMOTE_APP_ID, 0),
            get_byte_16!(REMOTE_APP_ID, 1),
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 3),
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_PORT, 1),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Test an encoding when the provided buffer is too small (should panic).
    #[test]
    #[should_panic(expected = "assertion failed: buffer.len() >= len")]
    fn cqc_hdr_buf_too_small() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(Tp::Hello),
            app_id: 0,
            length: 0,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: None,
        };

        // Buffer to write into.
        let mut buffer = vec![0xFF; (CqcHdr::hdr_len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode_request(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC header (should panic).
    #[test]
    #[should_panic(expected = "assertion failed: buffer.len() >= len")]
    fn cmd_hdr_buf_too_small() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(Tp::Hello),
            app_id: 0,
            length: 0,
        };

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: 0,
            instr: Cmd::I,
            options: CmdOpt::empty(),
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: XtraHdr::None,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let mut buffer = vec![0xFF; (CqcHdr::hdr_len() + CmdHdr::hdr_len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode_request(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too large.  Excess should
    // be untouched.
    #[test]
    fn buf_too_large() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(Tp::Hello),
            app_id: 0,
            length: 0,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: None,
        };

        // Buffer to write into.
        let write_len: usize = CqcHdr::hdr_len() as usize;
        let buf_len: usize = write_len + 4;
        let mut buffer = vec![0xFF; buf_len as usize];

        // Big-endian
        let expected: Vec<u8> = vec![
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            // The rest should be untouched.
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), write_len);
        assert_eq!(buffer, expected);
    }
}
