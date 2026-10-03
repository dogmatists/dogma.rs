// This is free and unencumbered software released into the public domain.

#![doc = include_str!("usage.md")]
#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub mod prelude;

#[cfg(any(feature = "enums", any(feature = "iri", feature = "uri")))]
pub mod enums;
#[cfg(any(feature = "enums", any(feature = "iri", feature = "uri")))]
pub use enums::*;

mod features;
pub use features::*;

#[cfg(any(feature = "iri", feature = "uri"))]
pub mod structs;
#[cfg(any(feature = "iri", feature = "uri"))]
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

#[cfg(feature = "alloc")]
pub mod path;
#[cfg(feature = "alloc")]
pub use path::*;
