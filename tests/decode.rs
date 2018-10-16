extern crate cqc;

#[cfg(test)]
mod tests {
    use cqc::hdr::*;
    use cqc::{Decoder, Response, RspInfo};

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

    // Decode a response packet that only has a CQC header.
    #[test]
    fn cqc_hdr_decode() {
        let cqc_type = Tp::NewOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = 0;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: Version::V1,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        // The response.
        let response = Response {
            cqc_hdr,
            notify: RspInfo::None,
        };

        // Big-endian
        let packet: Vec<u8> = vec![
            Version::V1 as u8,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response packet that has CQC and Notify headers.
    #[test]
    fn notify_hdr_decode() {
        let cqc_type = Tp::NewOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = NotifyHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: Version::V1,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        // The Notify header.
        let notify_hdr = NotifyHdr {
            qubit_id: QUBIT_ID,
            remote_ap_id: 0,
            remote_node: 0,
            timestamp: 0,
            remote_port: 0,
            outcome: 0,
            align: 0,
        };

        // The response.
        let response = Response {
            cqc_hdr,
            notify: RspInfo::Notify(notify_hdr),
        };

        // Big-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            Version::V1 as u8,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Notify header.
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response packet that has CQC and Entanglement Info headers.
    #[test]
    fn ent_info_hdr_decode() {
        let cqc_type = Tp::EprOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = EntInfoHdr::hdr_len();

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: Version::V1,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        // The Entanglement Info header.
        let ent_info_hdr = EntInfoHdr {
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
        };

        // The response.
        let response = Response {
            cqc_hdr,
            notify: RspInfo::EntInfo(ent_info_hdr),
        };

        // Big-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            Version::V1 as u8,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
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

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result, response);
    }

    // Decode a response packet that only has a non-zero length indicating
    // follow-up headers, but it is too short to hold the expected header.
    // This should return an Error and thus panic on unwrap.
    #[test]
    #[should_panic(expected = "Response length insufficient to hold a Notify Header")]
    fn invalid_len_decode() {
        let cqc_type = Tp::NewOk;
        let length: u32 = NotifyHdr::hdr_len() - 1;

        let packet: Vec<u8> = vec![
            // CQC header.
            Version::V1 as u8,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
            // Notify header.
            get_byte_16!(QUBIT_ID, 0),
            get_byte_16!(QUBIT_ID, 1),
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        let decoder = Decoder::new();
        decoder.decode(&packet[..]).unwrap();
    }

    // Decode a response packet that only has an invalid CQC version.  This
    // should return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC version")]
    fn invalid_version_decode() {
        let cqc_type = Tp::NewOk;
        let length: u32 = 0;

        let packet: Vec<u8> = vec![
            Version::V1 as u8 + 1,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
        ];

        let decoder = Decoder::new();
        decoder.decode(&packet[..]).unwrap();
    }

    // Decode a response packet that only has an invalid message type.  This
    // should return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Invalid CQC message type")]
    fn invalid_msg_type_decode() {
        let length: u32 = 0;

        let packet: Vec<u8> = vec![
            Version::V1 as u8,
            0xFF,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let decoder = Decoder::new();
        decoder.decode(&packet[..]).unwrap();
    }
}
