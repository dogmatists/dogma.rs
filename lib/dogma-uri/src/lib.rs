// This is free and unencumbered software released into the public domain.

//! Validated URIs and IRIs with borrowed and owned representations.
//!
//! Enable `iri` for Unicode identifiers or `uri` for both IRI and ASCII URI
//! support. Both are enabled by default, along with `serde` and `std`.
//! String operations work with `no_std` and allocation. Filesystem and network
//! operations require `std`; Clap and Miette interop is opt-in.
//! The compatibility `enums` and `structs` features each enable both identifiers.
//!
//! The opt-in `url` feature enables checked conversions to and from
//! `url::Url`. Converting to a URL applies WHATWG normalization (including
//! IDNA host conversion); converting back validates its serialized spelling,
//! since WHATWG URLs can contain syntax rejected by RFC URIs and IRIs.
//!
//! Enable `iri-string` for outbound conversions to its validated string types.
//! Borrowing and moving owned strings are zero-copy; copying a borrowed value
//! into an owned string allocates. Existing inbound conversions and typed
//! accessors are always available with the corresponding identifier feature,
//! since `iri-string` also supplies the underlying storage and validation.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "iri")]
pub use known_schemes::IriScheme;
#[cfg(feature = "uri")]
pub use known_schemes::UriScheme;

#[cfg(feature = "iri")]
mod iri;
#[cfg(feature = "iri")]
pub use iri::*;

#[cfg(feature = "iri")]
mod iri_error;
#[cfg(feature = "iri")]
pub use iri_error::*;

#[cfg(feature = "iri")]
mod iri_authority;
#[cfg(feature = "iri")]
pub use iri_authority::*;

#[cfg(feature = "uri")]
mod uri;
#[cfg(feature = "uri")]
pub use uri::*;

#[cfg(feature = "uri")]
mod uri_error;
#[cfg(feature = "uri")]
pub use uri_error::*;

#[cfg(feature = "uri")]
mod uri_authority;
#[cfg(feature = "uri")]
pub use uri_authority::*;

#[cfg(feature = "url")]
#[path = "interop/url.rs"]
mod interop_url;

#[cfg(feature = "iri-string")]
#[path = "interop/iri_string.rs"]
mod interop_iri_string;
