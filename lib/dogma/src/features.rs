// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "collection")]
    "collection",
    #[cfg(feature = "countable")]
    "countable",
    #[cfg(feature = "iri")]
    "iri",
    #[cfg(feature = "labeled")]
    "labeled",
    #[cfg(feature = "named")]
    "named",
    #[cfg(feature = "unstable")]
    "unstable",
    #[cfg(feature = "uri")]
    "uri",
];
