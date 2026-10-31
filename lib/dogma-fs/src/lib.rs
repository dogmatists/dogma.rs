// This is free and unencumbered software released into the public domain.

//! Filesystem operations for Dogma paths.
//!
//! This crate builds on the path types of [`dogma_path`] and does not export
//! any items yet. Like the other Dogma crates, it builds with `#![no_std]`;
//! the `std` feature, enabled by default, forwards to `dogma-path` and will
//! gate operations that access the filesystem. `all` currently enables
//! nothing.

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
