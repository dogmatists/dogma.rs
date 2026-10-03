// This is free and unencumbered software released into the public domain.

//! Universally unique identifiers (UUIDs).
//!
//! [`Uuid`] is a 128-bit value with lossless conversions to and from 16 bytes.
//! It preserves every bit, including the version and variant fields, and can
//! represent any UUID version without enabling a version-specific feature.
//!
//! ```
//! use dogma_uuid::Uuid;
//!
//! let bytes = [
//!     0x67, 0xe5, 0x50, 0x44, 0x10, 0xb1, 0x42, 0x6f,
//!     0x92, 0x47, 0xbb, 0x68, 0x0e, 0x5f, 0xe0, 0xc8,
//! ];
//! let id = Uuid::from_bytes(bytes);
//! assert_eq!(id.as_bytes(), &bytes);
//! assert_eq!(<[u8; 16]>::from(id), bytes);
//! ```
//!
//! # Features
//!
//! The default features are `all` and `std`. `all` enables `alloc` and `serde`;
//! `--no-default-features` provides the core API without the standard library
//! or allocation. `alloc` enables conversion to a byte vector and is also
//! enabled by `std`. `serde` supports human-readable UUID strings and compact
//! binary serialization.
//!
//! # Compatibility
//!
//! Lossless [`From`] conversions support interoperability with [`::uuid::Uuid`].
//! The `std`, `arbitrary`, `atomic`, `borsh`, `bytemuck`, `serde`, `v1`, `v3`,
//! `v4`, `v5`, `v6`, `v7`, `v8`, and `zerocopy` features are forwarded to the
//! `uuid` crate. Apart from `serde`, these optional integrations and generation
//! APIs are currently accessed through that type. Random generation features
//! require a target supported by its random source; upstream configuration
//! requirements also apply to experimental integrations such as `zerocopy`.

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod uuid;

pub use uuid::Uuid;
