//! # CQC Decoder
//!
//! This module provides the decoder for the CQC protocol.

use hdr::*;
use std::result;
use {Request, Response};

pub enum Error {
    Version,
}

pub type Result<T> = result::Result<T, Error>;

pub enum CqcPacket {
    Request(Request),
    Response(Response),
}

pub fn decode(buffer: &[u8]) -> Result<Option<CqcPacket>> {
    Ok(None)
}
