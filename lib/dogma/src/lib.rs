// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use dogma::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub mod prelude;

#[cfg(any(feature = "enums", any(feature = "iri", feature = "uri")))]
pub mod enums;
#[cfg(any(feature = "enums", any(feature = "iri", feature = "uri")))]
pub use enums::*;

mod features;
pub use features::*;

#[cfg(any(feature = "structs", any(feature = "iri", feature = "uri")))]
pub mod structs;
#[cfg(any(feature = "structs", any(feature = "iri", feature = "uri")))]
pub use structs::*;

/// Common traits for objects.
#[cfg(any(
    feature = "traits",
    any(
        feature = "collection",
        feature = "countable",
        feature = "labeled",
        feature = "named"
    )
))]
pub mod traits;
#[cfg(any(
    feature = "traits",
    any(
        feature = "collection",
        feature = "countable",
        feature = "labeled",
        feature = "named"
    )
))]
pub use traits::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
