// This is free and unencumbered software released into the public domain.

#![doc = include_str!("usage.md")]
#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "iri")]
pub use dogma_uri as uri;
#[cfg(feature = "iri")]
pub use dogma_uri::*;

#[cfg(any(
    feature = "collection",
    feature = "countable",
    feature = "labeled",
    feature = "named"
))]
pub use dogma_traits as traits;
#[cfg(any(
    feature = "collection",
    feature = "countable",
    feature = "labeled",
    feature = "named"
))]
pub use dogma_traits::*;

// `dogma-fs` has no items yet; also glob-export it once it does.
#[cfg(feature = "fs")]
pub use dogma_fs as fs;

#[cfg(feature = "path")]
pub use dogma_path as path;
#[cfg(feature = "path")]
pub use dogma_path::*;

#[cfg(feature = "uuid")]
pub use dogma_uuid as uuid;
#[cfg(feature = "uuid")]
pub use dogma_uuid::*;

// Runs the README's Rust examples as doctests, in the repository and in the
// published package, which store the README at different relative paths.
#[cfg(all(doctest, feature = "named"))]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_PKG_README")))]
struct ReadmeDoctests;
