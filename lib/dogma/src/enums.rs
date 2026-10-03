// This is free and unencumbered software released into the public domain.

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

#[cfg(feature = "uri")]
mod uri;
#[cfg(feature = "uri")]
pub use uri::*;

#[cfg(feature = "uri")]
mod uri_error;
#[cfg(feature = "uri")]
pub use uri_error::*;
