// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use dogma::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
mod prelude;

mod feature;
pub use feature::*;

/// Common traits for objects.
pub mod traits {
    mod labeled;
    pub use labeled::*;

    mod maybe_labeled;
    pub use maybe_labeled::*;

    mod maybe_named;
    pub use maybe_named::*;

    mod named;
    pub use named::*;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
