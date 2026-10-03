// This is free and unencumbered software released into the public domain.

//! Ancestor paths and their conversion errors.
//!
//! Parsing and formatting work with `no_std` and allocation. Native path
//! conversions and filesystem queries require `std`; Camino support is opt-in.
//! The default features enable `serde` and `std`.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod ancestor_path;
pub use ancestor_path::*;

mod from_path_error;
pub use from_path_error::*;
