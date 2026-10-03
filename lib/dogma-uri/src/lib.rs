// This is free and unencumbered software released into the public domain.

//! Validated URIs and IRIs with borrowed and owned representations.
//!
//! Enable `iri` for Unicode identifiers or `uri` for both IRI and ASCII URI
//! support. Both are enabled by default, along with `serde` and `std`.
//! String operations work with `no_std` and allocation. Filesystem and network
//! operations require `std`; Clap and Miette integrations are opt-in.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "iri")]
pub mod enums;
#[cfg(feature = "iri")]
pub use enums::*;

#[cfg(feature = "iri")]
pub mod structs;
#[cfg(feature = "iri")]
pub use structs::*;
