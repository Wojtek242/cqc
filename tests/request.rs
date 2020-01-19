extern crate cqc;

#[cfg(test)]
mod request {
    use cqc::builder::{Builder, RemoteId};
    use cqc::hdr::*;
    use cqc::{Decoder, Encoder, Request};

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
    fn cqc_hdr() {
        let builder = Builder::new(APP_ID);
        let request = builder.hello();

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Hello);
        let length = 0;

        // Big-endian
        let expected: Vec<u8> = vec![
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let encoder = Encoder::new();
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Encode a packet that has a CMD header, but no XTRA header.
    #[test]
    fn cmd_hdr() {
        let builder = Builder::new(APP_ID);
        let request = builder
            .cmd_new(QUBIT_ID, *CmdOpt::empty().set_notify().set_block());

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len();
        let instr = Cmd::New;
        let options = *CmdOpt::empty().set_notify().set_block();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            From::from(msg_type),
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
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Encode a packet with a CMD and ROT headers.
    #[test]
    fn rot_hdr() {
        let builder = Builder::new(APP_ID);
        let request = builder.cmd_rot_x(
            QUBIT_ID,
            *CmdOpt::empty().set_notify().set_block(),
            STEP,
        );

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len() + RotHdr::hdr_len();
        let instr = Cmd::RotX;
        let options = *CmdOpt::empty().set_notify().set_block();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            From::from(msg_type),
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
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Encode a packet with a CMD and QUBIT headers.
    #[test]
    fn qubit_hdr() {
        let builder = Builder::new(APP_ID);
        let request = builder.cmd_cnot(
            QUBIT_ID,
            *CmdOpt::empty().set_notify().set_block(),
            EXTRA_QUBIT_ID,
        );

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len() + QubitHdr::hdr_len();
        let instr = Cmd::Cnot;
        let options = *CmdOpt::empty().set_notify().set_block();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            From::from(msg_type),
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
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Encode a packet with a CMD and COMM headers.
    #[test]
    fn comm_hdr() {
        let builder = Builder::new(APP_ID);
        let request = builder.cmd_send(
            QUBIT_ID,
            *CmdOpt::empty().set_notify().set_block(),
            RemoteId {
                remote_app_id: REMOTE_APP_ID,
                remote_port: REMOTE_PORT,
                remote_node: REMOTE_NODE,
            },
        );

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len() + CommHdr::hdr_len();
        let instr = Cmd::Send;
        let options = *CmdOpt::empty().set_notify().set_block();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            From::from(msg_type),
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
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_PORT, 1),
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 3),
        ];

        let encoder = Encoder::new();
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Test an encoding when the provided buffer is too small (should panic).
    #[test]
    #[should_panic(expected = "failed to write whole buffer")]
    fn cqc_hdr_buf_too_small() {
        let builder = Builder::new(APP_ID);
        let request = builder.hello();

        // Buffer to write into.
        let mut buffer = vec![0xAA; (request.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC header (should panic).
    #[test]
    #[should_panic(expected = "failed to write whole buffer")]
    fn cmd_hdr_buf_too_small() {
        let builder = Builder::new(APP_ID);
        let request = builder.cmd_i(QUBIT_ID, CmdOpt::empty());

        // Buffer to write into.
        let mut buffer = vec![0xAA; (request.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too large.  Excess should
    // be untouched.
    #[test]
    fn buf_too_large() {
        let builder = Builder::new(APP_ID);
        let request = builder.hello();

        // Buffer to write into.
        let write_len: usize = request.len() as usize;
        let buf_len: usize = write_len + 4;
        let mut buffer = vec![0xAA; buf_len as usize];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Hello);
        let length = 0;

        // Big-endian
        let expected: Vec<u8> = vec![
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // The rest should be untouched.
            0xAA,
            0xAA,
            0xAA,
            0xAA,
        ];

        let encoder = Encoder::new();
        encoder.encode(&request, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Request = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, request);
    }

    // Decode a request that only has a non-zero length indicating follow-up
    // headers, but it is too short to hold the expected header. This should
    // return an Error and thus panic on unwrap.
    #[test]
    #[should_panic(expected = "invalid length 3, expected CmdHdr")]
    fn invalid_len() {
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len() - 1;
        let instr = Cmd::New;
        let options = *CmdOpt::empty().set_notify().set_block();

        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            From::from(msg_type),
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

        let decoder = Decoder::new();
        let _: Request = decoder.decode(&expected[..]).unwrap();
    }

    // Decode a request that only has an invalid CQC version. This should
    // return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC version")]
    fn invalid_version() {
        let msg_type = MsgType::Tp(Tp::Command);
        let length = CmdHdr::hdr_len();
        let instr = Cmd::New;
        let options = *CmdOpt::empty().set_notify().set_block();

        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8 + 1,
            From::from(msg_type),
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

        let decoder = Decoder::new();
        let _: Request = decoder.decode(&expected[..]).unwrap();
    }

    // Decode a request that only has an invalid message type. This should
    // return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC message type")]
    fn invalid_msg_type() {
        let length = CmdHdr::hdr_len();
        let instr = Cmd::New;
        let options = *CmdOpt::empty().set_notify().set_block();

        let expected: Vec<u8> = vec![
            // CQC header
            Version::V2 as u8,
            0xFF,
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

        let decoder = Decoder::new();
        let _: Request = decoder.decode(&expected[..]).unwrap();
    }
}
