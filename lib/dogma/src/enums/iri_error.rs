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

    /// A Windows verbatim or device namespace prefix cannot be represented.
    #[cfg(feature = "std")]
    PathPrefixUnsupported(std::path::PathBuf),
}

impl core::error::Error for IriError {}

impl fmt::Display for IriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            IriError::PathPrefixUnsupported(path) => {
                write!(f, "path prefix is not supported: {}", path.display())
            }
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

/// An error returned by [`Iri::try_to_path`](crate::Iri::try_to_path).
#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
#[non_exhaustive]
pub enum IriToPathError {
    /// The IRI scheme is not `file`.
    UnsupportedScheme,

    /// The authority is neither absent, empty, nor bare `localhost`.
    UnsupportedAuthority,

    /// A query component is present, even if empty.
    UnsupportedQuery,

    /// A fragment component is present, even if empty.
    UnsupportedFragment,

    /// The IRI path is empty or does not start with a literal `/`.
    /// On Windows, also returned if the decoded path lacks a drive root.
    PathNotAbsolute,

    /// The path cannot be percent-decoded as UTF-8.
    InvalidEncoding,

    /// The decoded path contains a NUL byte.
    NulByte,

    /// The path contains an encoded native separator (`/`, or `\` on Windows).
    EncodedSeparator,
}

#[cfg(feature = "std")]
impl core::error::Error for IriToPathError {}

#[cfg(feature = "std")]
impl fmt::Display for IriToPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedScheme => "filesystem path conversion requires the file scheme",
            Self::UnsupportedAuthority => "file-IRI authority is not supported",
            Self::UnsupportedQuery => "file-IRI queries are not supported",
            Self::UnsupportedFragment => "file-IRI fragments are not supported",
            Self::PathNotAbsolute => "file-IRI path does not identify an absolute native path",
            Self::InvalidEncoding => "file-IRI path cannot be decoded as UTF-8",
            Self::NulByte => "file-IRI path contains a NUL byte",
            Self::EncodedSeparator => "file-IRI path contains an encoded native separator",
        })
    }
}
