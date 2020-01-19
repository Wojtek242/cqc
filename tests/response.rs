extern crate cqc;

#[cfg(test)]
mod response {
    use cqc::builder::Server;
    use cqc::hdr::*;
    use cqc::{Decoder, Encoder, Response};

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

    macro_rules! get_byte_64 {
        ($value:expr, $byte:expr) => {
            ($value >> ((7 - $byte) * 8)) as u8
        };
    }

    // Set up constants.
    const QUBIT_ID: u16 = 0xFA_CE;
    const APP_ID: u16 = 0x0A_0E;
    const NODE: u32 = 0x12_34_AB_CD;
    const PORT: u16 = 0x91_03;
    const REMOTE_APP_ID: u16 = 0x5E_3F;
    const REMOTE_NODE: u32 = 0xAE_04_E2_52;
    const REMOTE_PORT: u16 = 0x20_43;
    const ENT_ID: u32 = 0x76_23_AE_9F;
    const TIMESTAMP: u64 = 0x22_11_AA_76_EA_82_9A_99;
    const TOG: u64 = 0x11_00_99_65_D9_71_89_88;
    const GOODNESS: u16 = 0xFF_01;

    // Decode a response that only has a CQC header.
    #[test]
    fn cqc_hdr() {
        let server = Server::new(APP_ID);
        let response = server.done();

        // Buffer to write into.
        let buf_len: usize = response.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Done);
        let length: u32 = 0;

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
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let result: Response = decoder.decode(&expected[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response with an Extra Qubit header.
    #[test]
    fn qubit_rsp() {
        let server = Server::new(APP_ID);
        let response = server.new_ok(QUBIT_ID);

        // Buffer to write into.
        let buf_len: usize = response.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::NewOk);
        let length: u32 = QubitHdr::hdr_len();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header.
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Qubit header.
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
        ];

        let encoder = Encoder::new();
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let result: Response = decoder.decode(&expected[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response with a Measurement Outcome header.
    #[test]
    fn meas_out_rsp() {
        let server = Server::new(APP_ID);
        let response = server.meas_out(MeasOut::One);

        // Buffer to write into.
        let buf_len: usize = response.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::MeasOut);
        let length: u32 = MeasOutHdr::hdr_len();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header.
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Notify header.
            0x01,
        ];

        let encoder = Encoder::new();
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let result: Response = decoder.decode(&expected[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response that has CQC and Entanglement Info headers.
    #[test]
    fn ent_info_hdr() {
        let server = Server::new(APP_ID);
        let response = server.epr_ok(
            QUBIT_ID,
            EntInfoHdr {
                node_a: NODE,
                port_a: PORT,
                app_id_a: APP_ID,
                node_b: REMOTE_NODE,
                port_b: REMOTE_PORT,
                app_id_b: REMOTE_APP_ID,
                id_ab: ENT_ID,
                timestamp: TIMESTAMP,
                tog: TOG,
                goodness: GOODNESS,
                df: 0,
                align: 0,
            },
        );

        // Buffer to write into.
        let buf_len: usize = response.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::EprOk);
        let length: u32 = QubitHdr::hdr_len() + EntInfoHdr::hdr_len();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header.
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Qubit header.
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            // Entanglement Info header.
            get_byte_32!(NODE, 0),
            get_byte_32!(NODE, 1),
            get_byte_32!(NODE, 2),
            get_byte_32!(NODE, 3),
            get_byte_16!(PORT, 0),
            get_byte_16!(PORT, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 3),
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_PORT, 1),
            get_byte_16!(REMOTE_APP_ID, 0),
            get_byte_16!(REMOTE_APP_ID, 1),
            get_byte_32!(ENT_ID, 0),
            get_byte_32!(ENT_ID, 1),
            get_byte_32!(ENT_ID, 2),
            get_byte_32!(ENT_ID, 3),
            get_byte_64!(TIMESTAMP, 0),
            get_byte_64!(TIMESTAMP, 1),
            get_byte_64!(TIMESTAMP, 2),
            get_byte_64!(TIMESTAMP, 3),
            get_byte_64!(TIMESTAMP, 4),
            get_byte_64!(TIMESTAMP, 5),
            get_byte_64!(TIMESTAMP, 6),
            get_byte_64!(TIMESTAMP, 7),
            get_byte_64!(TOG, 0),
            get_byte_64!(TOG, 1),
            get_byte_64!(TOG, 2),
            get_byte_64!(TOG, 3),
            get_byte_64!(TOG, 4),
            get_byte_64!(TOG, 5),
            get_byte_64!(TOG, 6),
            get_byte_64!(TOG, 7),
            get_byte_16!(GOODNESS, 0),
            get_byte_16!(GOODNESS, 1),
            0x00,
            0x00,
        ];

        let encoder = Encoder::new();
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let result: Response = decoder.decode(&expected[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response with a Measurement Outcome header.
    #[test]
    fn inf_time_rsp() {
        let server = Server::new(APP_ID);
        let response = server.inf_time(TIMESTAMP);

        // Buffer to write into.
        let buf_len: usize = response.len() as usize;
        let mut buffer = vec![0xAA; buf_len];

        // Expected values
        let msg_type = MsgType::Tp(Tp::InfTime);
        let length: u32 = TimeInfoHdr::hdr_len();

        // Big-endian
        let expected: Vec<u8> = vec![
            // CQC header.
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Notify header.
            get_byte_64!(TIMESTAMP, 0),
            get_byte_64!(TIMESTAMP, 1),
            get_byte_64!(TIMESTAMP, 2),
            get_byte_64!(TIMESTAMP, 3),
            get_byte_64!(TIMESTAMP, 4),
            get_byte_64!(TIMESTAMP, 5),
            get_byte_64!(TIMESTAMP, 6),
            get_byte_64!(TIMESTAMP, 7),
        ];

        let encoder = Encoder::new();
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let result: Response = decoder.decode(&expected[..]).unwrap();
        assert_eq!(result, response);
    }

    // Test an encoding when the provided buffer is too small (should panic).
    #[test]
    #[should_panic(expected = "failed to write whole buffer")]
    fn cqc_hdr_buf_too_small() {
        let server = Server::new(APP_ID);
        let response = server.done();

        // Buffer to write into.
        let mut buffer = vec![0xAA; (response.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&response, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too small, but sufficient
    // for the CQC header (should panic).
    #[test]
    #[should_panic(expected = "failed to write whole buffer")]
    fn cmd_hdr_buf_too_small() {
        let server = Server::new(APP_ID);
        let response = server.new_ok(QUBIT_ID);

        // Buffer to write into.
        let mut buffer = vec![0xAA; (response.len() - 1) as usize];

        let encoder = Encoder::new();

        // This should panic.
        encoder.encode(&response, &mut buffer[..]);
    }

    // Test an encoding when the provided buffer is too large.  Excess should
    // be untouched.
    #[test]
    fn buf_too_large() {
        let server = Server::new(APP_ID);
        let response = server.done();

        // Buffer to write into.
        let write_len: usize = response.len() as usize;
        let buf_len: usize = write_len + 4;
        let mut buffer = vec![0xAA; buf_len as usize];

        // Expected values
        let msg_type = MsgType::Tp(Tp::Done);
        let length: u32 = 0;

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
        encoder.encode(&response, &mut buffer[..]);
        assert_eq!(buffer, expected);

        let decoder = Decoder::new();
        let decoded: Response = decoder.decode(&buffer[..]).unwrap();
        assert_eq!(decoded, response);
    }

    // Decode a response that only has a non-zero length indicating follow-up
    // headers, but it is too short to hold the expected header. This should
    // return an Error and thus panic on unwrap.
    #[test]
    #[should_panic(expected = "invalid length 1, expected QubitHdr")]
    fn invalid_len() {
        let msg_type = MsgType::Tp(Tp::NewOk);
        let length: u32 = QubitHdr::hdr_len() - 1;

        let expected: Vec<u8> = vec![
            // CQC header.
            Version::V2 as u8,
            From::from(msg_type),
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Notify header.
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
        ];

        let decoder = Decoder::new();
        let _: Response = decoder.decode(&expected[..]).unwrap();
    }

    // Decode a response that only has an invalid CQC version. This should
    // return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC version")]
    fn invalid_version() {
        let msg_type = MsgType::Tp(Tp::Done);
        let length: u32 = 0;

        let expected: Vec<u8> = vec![
            Version::V2 as u8 + 1,
            From::from(msg_type),
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
        ];

        let decoder = Decoder::new();
        let _: Response = decoder.decode(&expected[..]).unwrap();
    }

    // Decode a response that only has an invalid message type. This should
    // return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC message type")]
    fn invalid_msg_type() {
        let length: u32 = 0;

        let expected: Vec<u8> = vec![
            Version::V2 as u8,
            0xFF,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let decoder = Decoder::new();
        let _: Response = decoder.decode(&expected[..]).unwrap();
    }
}
