//! # CQC Encoder
//!
//! This module provides the encoder for the CQC protocol.  The encoder does
//! not check for protocol correctness.

extern crate bincode;

use hdr::*;
use {ReqCmd, Request};

pub struct Encoder {
    config: bincode::Config,
}

impl Encoder {
    /// Create an encoder with the default endianness setting (little endian).
    pub fn new() -> Encoder {
        Encoder {
            config: bincode::config(),
        }
    }

    /// Create a big endian encoder.
    pub fn big_endian() -> Encoder {
        let mut config = bincode::config();
        config.big_endian();

        Encoder { config }
    }

    /// Create a little endian encoder.
    pub fn little_endian() -> Encoder {
        let mut config = bincode::config();
        config.little_endian();

        Encoder { config }
    }

    /// Encode a CQC request packet into buffer of bytes.  The return value is
    /// a the number of bytes written.
    ///
    /// If the provided buffer is not large enough to encode the request
    /// `encode_request` will panic.
    ///
    /// Currently, this only supports encoding of complete packets.  That is,
    /// partial packets cannot be encoded.
    pub fn encode_request<'buf>(&self, request: &Request, buffer: &'buf mut [u8]) -> usize {
        let mut pos: usize;
        let mut end: usize;

        end = CQC_HDR_LENGTH as usize;
        assert!(buffer.len() >= end);
        self.config
            .serialize_into(&mut buffer[..end], &request.cqc_hdr)
            .unwrap();
        pos = end;

        if request.req_cmd.is_none() {
            return end;
        }

        let req_cmd: &ReqCmd = request.req_cmd.as_ref().unwrap();

        end += CMD_HDR_LENGTH as usize;
        assert!(buffer.len() >= end);
        self.config
            .serialize_into(&mut buffer[pos..end], &req_cmd.cmd_hdr)
            .unwrap();
        pos = end;

        if req_cmd.xtra_hdr.is_none() {
            return end;
        }

        let xtra_hdr: &XtraHdr = &req_cmd.xtra_hdr.as_ref().unwrap();

        end += XTRA_HDR_LENGTH as usize;
        assert!(buffer.len() >= end);
        self.config
            .serialize_into(&mut buffer[pos..end], xtra_hdr)
            .unwrap();

        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    const REMOTE_APP_ID: u16 = 0x5E_3F;
    const REMOTE_NODE: u32 = 0xAE_04_E2_52;
    const REMOTE_PORT: u16 = 0x91_03;

    // Encode a request packet that only has a CQC header.
    #[test]
    fn cqc_hdr_encode() {
        let cqc_type = CqcTp::Hello;
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
        let buf_len: usize = (CQC_HDR_LENGTH + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Little-endian
        let expected: Vec<u8> = vec![
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
        ];

        let encoder = Encoder::little_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

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

        let encoder = Encoder::big_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet that has a CMD header, but no XTRA header.
    #[test]
    fn cmd_hdr_encode() {
        let cqc_type = CqcTp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CMD_HDR_LENGTH;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::New;
        let options = CMD_OPT_NOTIFY | CMD_OPT_BLOCK;

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: None,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CQC_HDR_LENGTH + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Little-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
            // CMD header
            get_byte_16!(QUBIT_ID, 1),
            get_byte_16!(QUBIT_ID, 0),
            instr as u8,
            options,
        ];

        let encoder = Encoder::little_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

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
            options,
        ];

        let encoder = Encoder::big_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and XTRA headers.
    #[test]
    fn xtra_hdr_encode() {
        let cqc_type = CqcTp::Command;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = CMD_HDR_LENGTH + XTRA_HDR_LENGTH;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        let instr = Cmd::Send;
        let options = CMD_OPT_NOTIFY | CMD_OPT_BLOCK;

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: QUBIT_ID,
            instr: instr,
            options: options,
        };

        // The XTRA header.
        let xtra_hdr = XtraHdr {
            xtra_qubit_id: 0,
            remote_app_id: REMOTE_APP_ID,
            remote_node: REMOTE_NODE,
            cmd_length: 0,
            remote_port: REMOTE_PORT,
            steps: 0,
            align: 0,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: Some(xtra_hdr),
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let buf_len: usize = (CQC_HDR_LENGTH + length) as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Little-endian
        let expected: Vec<u8> = vec![
            // CQC header
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
            // CMD header
            get_byte_16!(QUBIT_ID, 1),
            get_byte_16!(QUBIT_ID, 0),
            instr as u8,
            options,
            // XTRA header
            0x00,
            0x00,
            get_byte_16!(REMOTE_APP_ID, 1),
            get_byte_16!(REMOTE_APP_ID, 0),
            get_byte_32!(REMOTE_NODE, 3),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 0),
            0x00,
            0x00,
            0x00,
            0x00,
            get_byte_16!(REMOTE_PORT, 1),
            get_byte_16!(REMOTE_PORT, 0),
            0x00,
            0x00,
        ];

        let encoder = Encoder::little_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

        let encoder = Encoder::new();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);

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
            options,
            // XTRA header
            0x00,
            0x00,
            get_byte_16!(REMOTE_APP_ID, 0),
            get_byte_16!(REMOTE_APP_ID, 1),
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 3),
            0x00,
            0x00,
            0x00,
            0x00,
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_PORT, 1),
            0x00,
            0x00,
        ];

        let encoder = Encoder::big_endian();
        assert_eq!(encoder.encode_request(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Test an encoding when the provided buffer is too small (should panic).
    #[test]
    #[should_panic]
    fn cqc_hdr_buf_too_small() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(CqcTp::Hello),
            app_id: 0,
            length: 0,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: None,
        };

        // Buffer to write into.
        let mut buffer = vec![0xFF; (CQC_HDR_LENGTH - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode_request(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC header (should panic).
    #[test]
    #[should_panic]
    fn cmd_hdr_buf_too_small() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(CqcTp::Hello),
            app_id: 0,
            length: 0,
        };

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: 0,
            instr: Cmd::I,
            options: 0,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: None,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let mut buffer = vec![0xFF; (CQC_HDR_LENGTH + CMD_HDR_LENGTH - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode_request(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC and CMD headers (should panic).
    #[test]
    #[should_panic]
    fn xtra_hdr_buf_too_small() {
        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: 0,
            msg_type: MsgType::Tp(CqcTp::Hello),
            app_id: 0,
            length: 0,
        };

        // The CMD header.
        let cmd_hdr = CmdHdr {
            qubit_id: 0,
            instr: Cmd::I,
            options: 0,
        };

        // The XTRA header.
        let xtra_hdr = XtraHdr {
            xtra_qubit_id: 0,
            remote_app_id: 0,
            remote_node: 0,
            cmd_length: 0,
            remote_port: 0,
            steps: 0,
            align: 0,
        };

        let req_cmd = ReqCmd {
            cmd_hdr,
            xtra_hdr: Some(xtra_hdr),
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: Some(req_cmd),
        };

        // Buffer to write into.
        let mut buffer =
            vec![0xFF; (CQC_HDR_LENGTH + CMD_HDR_LENGTH + XTRA_HDR_LENGTH - 1) as usize];

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
            msg_type: MsgType::Tp(CqcTp::Hello),
            app_id: 0,
            length: 0,
        };

        // The request.
        let request = Request {
            cqc_hdr,
            req_cmd: None,
        };

        // Buffer to write into.
        let write_len: usize = CQC_HDR_LENGTH as usize;
        let buf_len: usize = write_len + 4;
        let mut buffer = vec![0xFF; buf_len as usize];

        // Little-endian
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
