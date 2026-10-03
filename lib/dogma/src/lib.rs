// This is free and unencumbered software released into the public domain.

#![doc = include_str!("usage.md")]
#![no_std]
#![deny(unsafe_code)]

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

#[cfg(feature = "path")]
pub use dogma_path as path;
#[cfg(feature = "path")]
pub use dogma_path::*;

#[cfg(feature = "uuid")]
pub use dogma_uuid as uuid;
#[cfg(feature = "uuid")]
pub use dogma_uuid::*;
