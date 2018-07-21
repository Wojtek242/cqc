//! # CQC Encoder
//!
//! This module provides the encoder for the CQC protocol.  The encoder does
//! not check for protocol correctness.

use Request;

/// Encode a CQC request packet into buffer of bytes. The return value is a
/// reference to the buffer provided as input.
///
/// If the provided buffer is not large enough to encode the request
/// `encode_request` will panic.
///
/// Currently, this only supports encoding of complete packets.  That is,
/// partial packets cannot be encoded.
pub fn encode_request<'buf>(request: &Request, buffer: &'buf mut [u8]) -> &'buf mut [u8] {
    buffer
}
