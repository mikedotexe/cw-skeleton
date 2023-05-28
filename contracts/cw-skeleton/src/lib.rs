#![doc(html_logo_url = "https://mikedotexe.s3.us-west-2.amazonaws.com/heart-eva-beylin.jpeg")]
// #![allow(rustdoc::broken_intra_doc_links)]
//! For an overview, see the [README here](./README.md).

pub mod entry_points;
pub mod errors;
pub mod msgs;
pub mod state;

#[cfg(test)]
mod tests;

/// Contract name as it'll be stored according to the [cw2 dependency](https://crates.io/crates/cw2)
pub const CONTRACT_NAME: &str = "crates.io:cw-skeleton";
/// The version comes from the manifest file (Cargo.toml)
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
