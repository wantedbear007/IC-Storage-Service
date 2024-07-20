use candid::{Nat, Principal};
use ciborium::into_writer;
use ic_cdk::export_candid;
use ic_http_certification::HttpRequest;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::BTreeSet;
use num_traits::cast::ToPrimitive;

mod api_admin;
mod api_http;
mod api_init;
mod api_query;
mod api_update;
mod store;

mod types;
use api_http::*;
use types::*;
// use ic_oss_types::file::*;

mod bytes;
pub use bytes::*;


pub const MAX_CHUNK_SIZE: u32 = 256 * 1024;
pub const MAX_FILE_SIZE: u64 = 384 * 1024 * 1024 * 1024; // 384G
pub const MAX_FILE_SIZE_PER_CALL: u64 = 1024 * 2000; // should less than 2M

const MILLISECONDS: u64 = 1_000_000;

static ANONYMOUS: Principal = Principal::anonymous();

pub fn unwrap_trap<T, E: std::fmt::Debug>(res: Result<T, E>, msg: &str) -> T {
    match res {
        Ok(v) => v,
        Err(err) => ic_cdk::trap(&format!("{}, {:?}", msg, err)),
    }
}

fn is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller) {
        Ok(())
    } else {
        Err("user is not a controller".to_string())
    }
}

fn is_controller_or_manager() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller) || store::state::is_manager(&caller) {
        Ok(())
    } else {
        Err("user is not a controller or manager".to_string())
    }
}


pub fn format_error<T>(err: T) -> String
where
    T: std::fmt::Debug,
{
    format!("{:?}", err)
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut h = crc32fast::Hasher::new();
    h.update(data);
    h.finalize()
}



pub fn nat_to_u64(nat: &Nat) -> u64 {
    nat.0.to_u64().unwrap_or(0)
    // 0
}

// to_cbor_bytes returns the CBOR encoding of the given object that implements the Serialize trait.
pub fn to_cbor_bytes(obj: &impl Serialize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    into_writer(obj, &mut buf).expect("failed to encode in CBOR format");
    buf
}

// fn is_authenticated() -> Result<(), String> {
//     if ic_cdk::caller() == ANONYMOUS {
//         Err("anonymous user is not allowed".to_string())
//     } else {
//         Ok(())
//     }
// }

export_candid!();