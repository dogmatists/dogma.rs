// This is free and unencumbered software released into the public domain.

#[cfg(feature = "iri")]
mod iri_authority;
#[cfg(feature = "iri")]
pub use iri_authority::*;

#[cfg(feature = "uri")]
mod uri_authority;
#[cfg(feature = "uri")]
pub use uri_authority::*;
