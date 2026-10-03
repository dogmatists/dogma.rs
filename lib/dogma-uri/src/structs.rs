// This is free and unencumbered software released into the public domain.

//! Borrowed authority components for validated identifiers.
//!
//! The `structs` feature enables both authority types and their `iri` and `uri`
//! dependencies. Enable `iri` alone for just IRI support, or `uri` for both.
//! Network resolution additionally requires `std`.

#[cfg(feature = "iri")]
mod iri_authority;
#[cfg(feature = "iri")]
pub use iri_authority::*;

#[cfg(feature = "uri")]
mod uri_authority;
#[cfg(feature = "uri")]
pub use uri_authority::*;
