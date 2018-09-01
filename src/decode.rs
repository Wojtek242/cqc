//! # CQC Decoder
//!
//! This module provides the decoder for the CQC protocol.

extern crate bincode;

use hdr::*;
use std::result;
use std::fmt;
use std::error;
use {Request, Response, RspNotify};

/// An error in decoding.
///
/// # Possible errors
///
/// - Version - invalid version (MUST be <= 0).
/// - Deserialize - An error occurred while deserializing.
/// - Invalid - The packet is invalid.
#[derive(Debug)]
pub enum Error {
    Version(u8),
    Deserialize(Box<bincode::ErrorKind>),
    Invalid(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Version(_) => "Unsupported CQC version",
            &Error::Deserialize(_) => "Deserialization from binary format failed",
            &Error::Invalid(_) => "The packet is invalid",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Error::Version(ref ver) => write!(f, "Unsupported CQC version: {}", ver),
            &Error::Deserialize(ref ek) => ek.fmt(f),
            &Error::Invalid(ref s) => write!(f, "{}", s),
        }
    }
}

/// A result of any decoding action.  The `Ok` result is a tuple of bytes read
/// and a decoding `Status`.
pub type Result = result::Result<(usize, Status), Error>;

#[derive(Debug, PartialEq)]
pub enum CqcPacket {
    CqcHdr(CqcHdr),
    Request(Request),
    Response(Response),
}

impl CqcPacket {
    pub fn is_cqc_hdr(&self) -> bool {
        match self {
            &CqcPacket::CqcHdr(_) => true,
            _ => false,
        }
    }

    pub fn get_cqc_hdr(self) -> Option<CqcHdr> {
        match self {
            CqcPacket::CqcHdr(cqc_hdr) => Some(cqc_hdr),
            _ => None,
        }
    }

    pub fn is_request(&self) -> bool {
        match self {
            &CqcPacket::Request(_) => true,
            _ => false,
        }
    }

    pub fn get_request(self) -> Option<Request> {
        match self {
            CqcPacket::Request(request) => Some(request),
            _ => None,
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            &CqcPacket::Response(_) => true,
            _ => false,
        }
    }

    pub fn get_response(self) -> Option<Response> {
        match self {
            CqcPacket::Response(response) => Some(response),
            _ => None,
        }
    }
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
            &Status::Complete(..) => true,
            &Status::Partial => false,
        }
    }

    /// Convenience method to check if status is partial.
    #[inline]
    pub fn is_partial(&self) -> bool {
        match self {
            &Status::Complete(..) => false,
            &Status::Partial => true,
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
    pub fn decode(&self, buffer: &[u8]) -> Result {
        let (bytes, status) = self.decode_cqc_hdr(buffer)?;

        let cqc_hdr = match status {
            Status::Complete(CqcPacket::CqcHdr(pkt)) => pkt,
            Status::Partial => return Ok((0, Status::Partial)),
            _ => panic!(),
        };

        if cqc_hdr.length == 0 {
            return Ok((
                bytes,
                Status::Complete(CqcPacket::Response(Response {
                    cqc_hdr,
                    notify: None,
                })),
            ));
        }

        self.decode_notify(&buffer[bytes..], cqc_hdr)
    }

    /// Decode a CQC header.
    ///
    /// Returns a `Status` object if no error during parsing occurred.  If the
    /// data provided is incomplete and a CQC packet cannot be constructed a
    /// `Status::Partial` is returned.
    pub fn decode_cqc_hdr(&self, buffer: &[u8]) -> Result {
        let end: usize = CQC_HDR_LENGTH as usize;

        if buffer.len() >= end {
            let cqc_hdr: CqcHdr = match self.config.deserialize_from(&buffer[..end]) {
                Ok(result) => result,
                Err(e) => return Err(Error::Deserialize(e)),
            };

            if cqc_hdr.version != CQC_VERSION {
                return Err(Error::Version(cqc_hdr.version));
            }

            return Ok((
                CQC_HDR_LENGTH as usize,
                Status::Complete(CqcPacket::CqcHdr(cqc_hdr)),
            ));
        }

        Ok((0, Status::Partial))
    }

    /// Decode a Notify or Entanglement Info header.
    ///
    /// Returns a `Status` object if no error during parsing occurred.  If the
    /// data provided is incomplete and a CQC packet cannot be constructed a
    /// `Status::Partial` is returned.
    pub fn decode_notify(&self, buffer: &[u8], cqc_hdr: CqcHdr) -> Result {
        let (msg_type, length) = (cqc_hdr.msg_type, cqc_hdr.length);

        match msg_type {
            MsgType::Tp(Tp::Recv) | MsgType::Tp(Tp::Measout) | MsgType::Tp(Tp::NewOk) => {
                if length < NOTIFY_HDR_LENGTH {
                    return Err(Error::Invalid(format!(
                        "Need at least {} bytes for Notify Header, packet has {}",
                        NOTIFY_HDR_LENGTH, length
                    )));
                }

                let end = NOTIFY_HDR_LENGTH as usize;
                if buffer.len() >= end {
                    match self.config.deserialize_from(&buffer[..end]) {
                        Ok(result) => {
                            return Ok((
                                (CQC_HDR_LENGTH + length) as usize,
                                Status::Complete(CqcPacket::Response(Response {
                                    cqc_hdr,
                                    notify: Some(RspNotify::Notify(result)),
                                })),
                            ));
                        }
                        Err(e) => return Err(Error::Deserialize(e)),
                    };
                }
            }
            MsgType::Tp(Tp::EprOk) => {
                if length < ENT_INFO_HDR_LENGTH {
                    return Err(Error::Invalid(format!(
                        "Need at least {} bytes for Entanglement Info, packet has {}",
                        ENT_INFO_HDR_LENGTH, length
                    )));
                }

                let end = ENT_INFO_HDR_LENGTH as usize;
                if buffer.len() >= end {
                    match self.config.deserialize_from(&buffer[..end]) {
                        Ok(result) => {
                            return Ok((
                                (CQC_HDR_LENGTH + length) as usize,
                                Status::Complete(CqcPacket::Response(Response {
                                    cqc_hdr,
                                    notify: Some(RspNotify::EntInfo(result)),
                                })),
                            ));
                        }
                        Err(e) => return Err(Error::Deserialize(e)),
                    };
                }
            }
            _ => {
                return Ok((
                    (CQC_HDR_LENGTH + length) as usize,
                    Status::Complete(CqcPacket::Response(Response {
                        cqc_hdr,
                        notify: None,
                    })),
                ));
            }
        }

        // If we reached here, there is nothing wrong with the data, but the
        // packet is incomplete.
        Ok((0, Status::Partial))
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
        let cqc_type = Tp::NewOk;
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
        let cqc_type = Tp::NewOk;
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
        let cqc_type = Tp::EprOk;
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
        let cqc_type = Tp::NewOk;
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
    // follow-up headers, but it is too short to hold the expected header.
    // This should return an Error and thus panic on unwrap.
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Invalid")]
    fn invalid_type_decode() {
        let cqc_type = Tp::NewOk;
        let length: u32 = NOTIFY_HDR_LENGTH - 1;

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

    // Decode a response packet that only has an invalid message type.  This
    // should return an error (and thus panic on an unwrap).
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Deserialize(Custom")]
    fn invalid_msg_type_decode() {
        let length: u32 = 0;

        let packet: Vec<u8> = vec![
            CQC_VERSION + 1,
            0xFF,
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
}
