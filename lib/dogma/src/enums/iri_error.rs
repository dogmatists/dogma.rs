// This is free and unencumbered software released into the public domain.

#![allow(unused)]
use crate::prelude::{fmt, String};

pub type IriResult<T> = core::result::Result<T, IriError>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum IriError {
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::iri::invalid),
            help("it seems that the IRI is malformed in some way"),
            url(docsrs),
        )
    )]
    Invalid(String),
}

impl core::error::Error for IriError {}

impl fmt::Display for IriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IriError::Invalid(s) => write!(f, "invalid IRI: {}", s),
        }
    }
}
