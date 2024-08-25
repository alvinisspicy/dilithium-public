#[cfg(feature = "aes")]
mod aes256ctr;
mod api;
mod fips202;
mod ntt;
mod packing;
mod params;
mod poly;
mod polyvec;
mod randombytes;
mod reduce;
mod rounding;
pub mod sign;
mod symmetric;
pub use params::*;

pub use api::*;

#[cfg(feature = "wasm")]
mod wasm;

pub use sign::{
  crypto_sign_keypair as sign_keypair, crypto_sign_signature as sign_signature,
  crypto_sign_verify as sign_verify,
};
