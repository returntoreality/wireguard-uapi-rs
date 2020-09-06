use std::convert::TryFrom;
use std::convert::TryInto;
use std::mem::size_of;

mod attr;
pub mod genl;
mod linux;
mod message;
mod utils;

use genl::*;
use message::*;
use utils::*;

fn serialize<T: NetlinkPayloadRequest>(message: &NetlinkMessageRequest<T>) -> Vec<u8> {
    let mut bytes = vec![];
    message.serialize(&mut bytes);
    return bytes;
}

fn write_to_buf_with_prefixed_u32_len<F>(mut buf: &mut Vec<u8>, write: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    let num_bytes_before = buf.len();
    buf.extend_from_slice(&[0u8; size_of::<u32>()]);
    let len_bytes_range = num_bytes_before..buf.len();

    write(&mut buf);

    let num_bytes_after = buf.len();
    // TODO: Propagate this error properly
    let message_len = u32::try_from(num_bytes_after - num_bytes_before).unwrap();

    buf.splice(len_bytes_range, message_len.to_ne_bytes().iter().cloned());
}

fn write_to_buf_with_prefixed_u16_len<F>(mut buf: &mut Vec<u8>, write: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    let num_bytes_before = buf.len();
    buf.extend_from_slice(&[0u8; size_of::<u16>()]);
    let len_bytes_range = num_bytes_before..buf.len();
    println!("{:?}", len_bytes_range);

    write(&mut buf);

    let num_bytes_after = buf.len();
    // TODO: Propagate this error properly
    let message_len = u16::try_from(num_bytes_after - num_bytes_before).unwrap();

    buf.splice(len_bytes_range, message_len.to_ne_bytes().iter().cloned());
}
