//! # CQC Encoder
//!
//! This module provides the encoder for the CQC protocol.

use Request;

pub fn encode_request<'b>(request: &Request, buffer: &'b mut [u8]) -> &'b mut [u8] {
    buffer
}
