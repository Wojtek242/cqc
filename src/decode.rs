//! # CQC Decoder
//!
//! This module provides the decoder for the CQC protocol.

extern crate bincode;

use hdr::*;
use std::result;
use {Request, Response, RspNotify};

/// An error in decoding.
///
/// # Possible errors
///
/// - Version - invalid version (MUST be <= 0).
/// - Deserialize - An error occurred while deserializing.
#[derive(Debug)]
pub enum Error {
    Version,
    Deserialize,
}

/// A result of any decoding action.  The `Ok` result is a tuple of bytes read
/// and a decoding `Status`.
pub type Result = result::Result<(usize, Status), Error>;

#[derive(Debug, PartialEq)]
pub enum CqcPacket {
    Request(Request),
    Response(Response),
}

/// The result of a successful decode pass.
///
/// - `Complete` is used when enough data was provided for a complete packet.
/// - `Partial` is used when there was not enough data to decode an entire
/// packet, but no invalid data was found.
#[derive(Debug)]
pub enum Status {
    Complete(CqcPacket),
    Partial,
}

impl Status {
    /// Convenience method to check if status is complete.
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Status::Complete(..) => true,
            Status::Partial => false,
        }
    }

    /// Convenience method to check if status is partial.
    #[inline]
    pub fn is_partial(&self) -> bool {
        match self {
            Status::Complete(..) => false,
            Status::Partial => true,
        }
    }

    /// Convenience method to unwrap a Complete value.  Panics if the status is
    /// `Partial`.
    pub fn unwrap(self) -> CqcPacket {
        match self {
            Status::Complete(cqc_packet) => cqc_packet,
            Status::Partial => panic!("Tried to unwrap Status::Partial"),
        }
    }
}

/// Convenience functions for reading bitwise options.
pub trait GetOpts {
    /// Convenience function to get the notify bit-flag.
    fn get_opt_notify(&self) -> bool;
    /// Convenience function to get the action bit-flag.
    fn get_opt_action(&self) -> bool;
    /// Convenience function to get the block bit-flag.
    fn get_opt_block(&self) -> bool;
    /// Convenience function to get the if-then bit-flag.
    fn get_opt_ifthen(&self) -> bool;
}

impl GetOpts for u8 {
    #[inline]
    fn get_opt_notify(&self) -> bool {
        (self & CMD_OPT_NOTIFY) != 0
    }

    #[inline]
    fn get_opt_action(&self) -> bool {
        (self & CMD_OPT_ACTION) != 0
    }

    #[inline]
    fn get_opt_block(&self) -> bool {
        (self & CMD_OPT_BLOCK) != 0
    }

    #[inline]
    fn get_opt_ifthen(&self) -> bool {
        (self & CMD_OPT_IFTHEN) != 0
    }
}

/// Packet decoder.
///
/// Note that currently only the decoding of complete packets is supported.
pub struct Decoder {
    config: bincode::Config,
}

impl Decoder {
    /// Create a `Decoder` with default endianness setting (little endian).
    pub fn new() -> Decoder {
        Decoder {
            config: bincode::config(),
        }
    }

    /// Create a big endian `Decoder`.
    pub fn big_endian() -> Decoder {
        let mut config = bincode::config();
        config.big_endian();

        Decoder { config }
    }

    /// Create a little endian `Decoder`.
    pub fn little_endian() -> Decoder {
        let mut config = bincode::config();
        config.little_endian();

        Decoder { config }
    }

    /// Decode supplied data.
    ///
    /// Returns a `Status` object if no error during parsing occurred.  If the
    /// data provided is incomplete and a CQC packet cannot be constructed a
    /// `Status::Partial` is returned.
    ///
    /// Note that currently only the decoding of complete response packets is
    /// supported.  Decode will panic otherwise.
    pub fn decode(&self, buffer: &[u8]) -> Result {
        let pos: usize;
        let mut end: usize;

        end = CQC_HDR_LENGTH as usize;
        assert!(buffer.len() >= end);
        let cqc_hdr: CqcHdr = match self.config.deserialize_from(&buffer[..end]) {
            Ok(result) => result,
            Err(_) => return Err(Error::Deserialize),
        };
        pos = end;

        if cqc_hdr.version != CQC_VERSION {
            return Err(Error::Version);
        }

        if cqc_hdr.length == 0 {
            return Ok((
                CQC_HDR_LENGTH as usize,
                Status::Complete(CqcPacket::Response(Response {
                    cqc_hdr,
                    notify: None,
                })),
            ));
        }

        match cqc_hdr.msg_type {
            MsgType::Tp(CqcTp::Recv) | MsgType::Tp(CqcTp::Measout) | MsgType::Tp(CqcTp::NewOk) => {
                end += NOTIFY_HDR_LENGTH as usize;
                assert!(buffer.len() >= end);
                assert!((CQC_HDR_LENGTH + cqc_hdr.length) as usize >= end);
                match self.config.deserialize_from(&buffer[pos..end]) {
                    Ok(result) => return Ok((
                        (CQC_HDR_LENGTH + cqc_hdr.length) as usize,
                        Status::Complete(CqcPacket::Response(Response {
                            cqc_hdr,
                            notify: Some(RspNotify::Notify(result)),
                        })),
                    )),
                    Err(_) => return Err(Error::Deserialize),
                };
            }
            MsgType::Tp(CqcTp::EprOk) => {
                end += ENT_INFO_HDR_LENGTH as usize;
                assert!(buffer.len() >= end);
                assert!((CQC_HDR_LENGTH + cqc_hdr.length) as usize >= end);
                match self.config.deserialize_from(&buffer[pos..end]) {
                    Ok(result) => return Ok((
                        (CQC_HDR_LENGTH + cqc_hdr.length) as usize,
                        Status::Complete(CqcPacket::Response(Response {
                            cqc_hdr,
                            notify: Some(RspNotify::EntInfo(result)),
                        })),
                    )),
                    Err(_) => return Err(Error::Deserialize),
                };
            }
            _ => panic!("Unexpected message type received: {:?}", cqc_hdr.msg_type),
        }
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
        let cqc_type = CqcTp::NewOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = 0;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
            msg_type: msg_type,
            app_id: APP_ID,
            length: length,
        };

        // The response.
        let response = CqcPacket::Response(Response {
            cqc_hdr,
            notify: None,
        });

        // Little-endian
        let packet: Vec<u8> = vec![
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
        ];

        let decoder = Decoder::little_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        // Big-endian
        let packet: Vec<u8> = vec![
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_32!(length, 0),
            get_byte_32!(length, 1),
            get_byte_32!(length, 2),
            get_byte_32!(length, 3),
        ];

        let decoder = Decoder::big_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);
    }

    // Decode a response packet that has CQC and Notify headers.
    #[test]
    fn notify_hdr_decode() {
        let cqc_type = CqcTp::NewOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = NOTIFY_HDR_LENGTH;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
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
        let response = CqcPacket::Response(Response {
            cqc_hdr,
            notify: Some(RspNotify::Notify(notify_hdr)),
        });

        // Little-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
            // Notify header.
            get_byte_16!(QUBIT_ID, 1),
            get_byte_16!(QUBIT_ID, 0),
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

        let decoder = Decoder::little_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        // Big-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            CQC_VERSION,
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

        let decoder = Decoder::big_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);
    }

    // Decode a response packet that has CQC and Entanglement Info headers.
    #[test]
    fn ent_info_hdr_decode() {
        let cqc_type = CqcTp::EprOk;
        let msg_type = MsgType::Tp(cqc_type);
        let length: u32 = ENT_INFO_HDR_LENGTH;

        // The CQC header.
        let cqc_hdr = CqcHdr {
            version: CQC_VERSION,
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
        let response = CqcPacket::Response(Response {
            cqc_hdr,
            notify: Some(RspNotify::EntInfo(ent_info_hdr)),
        });

        // Little-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
            // Entanglement Info header.
            get_byte_32!(NODE, 3),
            get_byte_32!(NODE, 2),
            get_byte_32!(NODE, 1),
            get_byte_32!(NODE, 0),
            get_byte_16!(PORT, 1),
            get_byte_16!(PORT, 0),
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(REMOTE_NODE, 3),
            get_byte_32!(REMOTE_NODE, 2),
            get_byte_32!(REMOTE_NODE, 1),
            get_byte_32!(REMOTE_NODE, 0),
            get_byte_16!(REMOTE_PORT, 1),
            get_byte_16!(REMOTE_PORT, 0),
            get_byte_16!(REMOTE_APP_ID, 1),
            get_byte_16!(REMOTE_APP_ID, 0),
            get_byte_32!(ENT_ID, 3),
            get_byte_32!(ENT_ID, 2),
            get_byte_32!(ENT_ID, 1),
            get_byte_32!(ENT_ID, 0),
            get_byte_64!(TIMESTAMP, 7),
            get_byte_64!(TIMESTAMP, 6),
            get_byte_64!(TIMESTAMP, 5),
            get_byte_64!(TIMESTAMP, 4),
            get_byte_64!(TIMESTAMP, 3),
            get_byte_64!(TIMESTAMP, 2),
            get_byte_64!(TIMESTAMP, 1),
            get_byte_64!(TIMESTAMP, 0),
            get_byte_64!(TOG, 7),
            get_byte_64!(TOG, 6),
            get_byte_64!(TOG, 5),
            get_byte_64!(TOG, 4),
            get_byte_64!(TOG, 3),
            get_byte_64!(TOG, 2),
            get_byte_64!(TOG, 1),
            get_byte_64!(TOG, 0),
            get_byte_16!(GOODNESS, 1),
            get_byte_16!(GOODNESS, 0),
            0x00,
            0x00,
        ];

        let decoder = Decoder::little_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        let decoder = Decoder::new();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);

        // Big-endian
        let packet: Vec<u8> = vec![
            // CQC header.
            CQC_VERSION,
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

        let decoder = Decoder::big_endian();
        let result = decoder.decode(&packet[..]).unwrap();
        assert_eq!(result.0, packet.len());
        assert_eq!(result.1.unwrap(), response);
    }

    // Decode a response packet that only has an invalid CQC version.  This
    // should return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Version")]
    fn invalid_version_decode() {
        let cqc_type = CqcTp::NewOk;
        let length: u32 = 0;

        let packet: Vec<u8> = vec![
            CQC_VERSION + 1,
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

    // Decode a response packet that only has a non-zero length indicating
    // follow-up headers, but the message type does not match any that are
    // expected to have follow-up headers.  This should return an error (and
    // thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "Unexpected message type received: Tp(Done)")]
    fn invalid_type_decode() {
        let cqc_type = CqcTp::Done;
        let length: u32 = NOTIFY_HDR_LENGTH;

        let packet: Vec<u8> = vec![
            // CQC header.
            CQC_VERSION,
            cqc_type as u8,
            get_byte_16!(APP_ID, 1),
            get_byte_16!(APP_ID, 0),
            get_byte_32!(length, 3),
            get_byte_32!(length, 2),
            get_byte_32!(length, 1),
            get_byte_32!(length, 0),
            // Notify header.
            get_byte_16!(QUBIT_ID, 1),
            get_byte_16!(QUBIT_ID, 0),
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
}
