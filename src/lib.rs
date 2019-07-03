use protobuf::types;
mod core;
pub mod protos;

pub use core::{protect_key, encrypt, decrypt};
