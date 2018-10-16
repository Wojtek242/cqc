extern crate cqc;

#[cfg(test)]
mod tests {
    use cqc::{Encoder, Request};
    use cqc::builder::RemoteId;
    use cqc::hdr::*;

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
        let request = Request::hello(APP_ID);

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Extract values
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);

        // Big-endian
        let expected: Vec<u8> = vec![
            Version::V1 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet that has a CMD header, but no XTRA header.
    #[test]
    fn cmd_hdr_encode() {
        let mut request = Request::command(APP_ID);
        request.cmd_new(QUBIT_ID, *CmdOpt::empty().set_notify().set_block());

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Extract values
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);
        let req_cmd = request.req_cmd.as_ref().unwrap();
        let (instr, options) = (req_cmd.cmd_hdr.instr, req_cmd.cmd_hdr.options);

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V1 as u8,
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
        assert_eq!(encoder.encode(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and ROT headers.
    #[test]
    fn rot_hdr_encode() {
        let mut request = Request::command(APP_ID);
        request.cmd_rot_x(QUBIT_ID, *CmdOpt::empty().set_notify().set_block(), STEP);

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Extract values
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);
        let req_cmd = request.req_cmd.as_ref().unwrap();
        let (instr, options) = (req_cmd.cmd_hdr.instr, req_cmd.cmd_hdr.options);

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V1 as u8,
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
        assert_eq!(encoder.encode(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and QUBIT headers.
    #[test]
    fn qubit_hdr_encode() {
        let mut request = Request::command(APP_ID);
        request.cmd_cnot(
            QUBIT_ID,
            *CmdOpt::empty().set_notify().set_block(),
            EXTRA_QUBIT_ID,
        );

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Extract values
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);
        let req_cmd = request.req_cmd.as_ref().unwrap();
        let (instr, options) = (req_cmd.cmd_hdr.instr, req_cmd.cmd_hdr.options);

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V1 as u8,
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
        assert_eq!(encoder.encode(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Encode a packet with a CMD and COMM headers.
    #[test]
    fn comm_hdr_encode() {
        let mut request = Request::command(APP_ID);
        request.cmd_send(
            QUBIT_ID,
            *CmdOpt::empty().set_notify().set_block(),
            RemoteId {
                remote_app_id: REMOTE_APP_ID,
                remote_node: REMOTE_NODE,
                remote_port: REMOTE_PORT,
            },
        );

        // Buffer to write into.
        let buf_len: usize = request.len() as usize;
        let mut buffer = vec![0xFF; buf_len];

        // Extract values
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);
        let req_cmd = request.req_cmd.as_ref().unwrap();
        let (instr, options) = (req_cmd.cmd_hdr.instr, req_cmd.cmd_hdr.options);

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header
            Version::V1 as u8,
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
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 3),
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_PORT, 1),
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode(&request, &mut buffer[..]), buf_len);
        assert_eq!(buffer, expected);
    }

    // Test an encoding when the provided buffer is too small (should panic).
    #[test]
    #[should_panic(expected = "assertion failed: buffer.len() >= len")]
    fn cqc_hdr_buf_too_small() {
        let request = Request::hello(APP_ID);

        // Buffer to write into.
        let mut buffer = vec![0xFF; (request.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC header (should panic).
    #[test]
    #[should_panic(expected = "assertion failed: buffer.len() >= len")]
    fn cmd_hdr_buf_too_small() {
        let mut request = Request::hello(APP_ID);
        request.cmd_i(QUBIT_ID, CmdOpt::empty());

        // Buffer to write into.
        let mut buffer = vec![0xFF; (request.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&request, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too large.  Excess should
    // be untouched.
    #[test]
    fn buf_too_large() {
        let request = Request::hello(APP_ID);

        // Buffer to write into.
        let write_len: usize = request.len() as usize;
        let buf_len: usize = write_len + 4;
        let mut buffer = vec![0xFF; buf_len as usize];

        // Extract values.
        let (msg_type, length) = (request.cqc_hdr.msg_type, request.cqc_hdr.length);

        // Big-endian
        let expected: Vec<u8> = vec![
            Version::V1 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // The rest should be untouched.
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];

        let encoder = Encoder::new();
        assert_eq!(encoder.encode(&request, &mut buffer[..]), write_len);
        assert_eq!(buffer, expected);
    }
}
