//! # CQC Decoder
//!
//! This module provides the decoder for the CQC protocol.

use hdr::*;
use std::result;
use {Request, Response};

/// An error in decoding.
///
/// # Possible errors
///
/// - Type - invalid message type.
/// - Version - invalid version (MUST be <= 0).
pub enum Error {
    Type,
    Version,
}

/// A result of any decoding action.  The `Ok` result is a tuple of bytes read
/// and a decoding `Status`.
pub type Result = result::Result<(usize, Status), Error>;

pub enum CqcPacket {
    Request(Request),
    Response(Response),
}

/// The result of a successful decode pass.
///
/// - `Complete` is used when enough data was provided for a complete packet.
/// - `Partial` is used when there was not enough data to decode an entire
/// packet, but no invalid data was found.
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
    // This struct is not used for anything at the moment, but will be
    // necessary when we want to hold partial packet state.
}

impl Decoder {

    /// Create and initialise a `Decoder`.
    pub fn new() -> Decoder {
        Decoder {}
    }

    /// Decode supplied data.
    ///
    /// Returns a `Status` object if no error during parsing occurred.  If the
    /// data provided is incomplete and a CQC packet cannot be constructed a
    /// `Status::Partial` is returned.
    ///
    /// Note that currently only the decoding of complete packets is supported.
    pub fn decode(buffer: &[u8]) -> Result {
        Ok((0, Status::Partial))
    }
}
