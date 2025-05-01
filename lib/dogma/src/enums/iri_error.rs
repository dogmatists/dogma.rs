// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

#[allow(unused)]
use crate::prelude::{fmt, format, String};

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
    Invalid(Option<String>),

    #[cfg(feature = "std")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::iri_error::path_is_relative),
            help("relative paths are not supported"),
            url(docsrs),
        )
    )]
    PathIsRelative(Option<std::path::PathBuf>),

    #[cfg(feature = "std")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::iri_error::path_not_unicode),
            help("non-Unicode paths are not supported"),
            url(docsrs),
        )
    )]
    PathNotUnicode(Option<std::path::PathBuf>),
}

impl core::error::Error for IriError {}

impl fmt::Display for IriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IriError::Invalid(None) => write!(f, "invalid IRI"),
            IriError::Invalid(Some(s)) => write!(f, "invalid IRI: {}", s),

            #[cfg(feature = "std")]
            IriError::PathIsRelative(None) => write!(f, "relative path is not supported"),
            #[cfg(feature = "std")]
            IriError::PathIsRelative(Some(path)) => {
                write!(f, "relative path is not supported: {}", path.display())
            }

            #[cfg(feature = "std")]
            IriError::PathNotUnicode(None) => write!(f, "non-Unicode path is not supported"),
            #[cfg(feature = "std")]
            IriError::PathNotUnicode(Some(path)) => {
                write!(f, "non-Unicode path is not supported: {}", path.display())
            }
        }
    }
}

impl From<iri_string::validate::Error> for IriError {
    fn from(_error: iri_string::validate::Error) -> Self {
        IriError::Invalid(None)
    }
}

impl From<iri_string::types::CreationError<String>> for IriError {
    fn from(error: iri_string::types::CreationError<String>) -> Self {
        IriError::Invalid(Some(error.into_source()))
    }
}
