// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "miette")]
use alloc::format;
use alloc::string::String;
use core::fmt;

/// A result with a URI construction error.
pub type UriResult<T> = core::result::Result<T, UriError>;

/// An error constructing a validated URI.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum UriError {
    /// Invalid syntax, retaining the input only for owned creation errors.
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::uri::invalid),
            help("it seems that the URI is malformed in some way"),
            url(docsrs),
        )
    )]
    Invalid(Option<String>),

    /// A relative filesystem path cannot identify an absolute URI.
    #[cfg(feature = "std")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::uri_error::path_is_relative),
            help("relative paths are not supported"),
            url(docsrs),
        )
    )]
    PathIsRelative(Option<std::path::PathBuf>),

    /// A filesystem path cannot be represented as Unicode.
    #[cfg(feature = "std")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::uri_error::path_not_unicode),
            help("non-Unicode paths are not supported"),
            url(docsrs),
        )
    )]
    PathNotUnicode(Option<std::path::PathBuf>),

    /// A Windows verbatim or device namespace prefix cannot be represented.
    #[cfg(feature = "std")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(dogma::uri_error::path_prefix_unsupported),
            help("verbatim and device namespace prefixes are not supported"),
            url(docsrs),
        )
    )]
    PathPrefixUnsupported(std::path::PathBuf),
}

impl core::error::Error for UriError {}

#[cfg(feature = "std")]
impl UriError {
    /// Preserves errors from the shared IRI filesystem constructor.
    pub(crate) fn from_iri(error: crate::IriError) -> Self {
        match error {
            crate::IriError::Invalid(input) => Self::Invalid(input),
            crate::IriError::PathIsRelative(path) => Self::PathIsRelative(path),
            crate::IriError::PathNotUnicode(path) => Self::PathNotUnicode(path),
            crate::IriError::PathPrefixUnsupported(path) => Self::PathPrefixUnsupported(path),
        }
    }
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(None) => f.write_str("invalid URI"),
            Self::Invalid(Some(text)) => write!(f, "invalid URI: {text}"),
            #[cfg(feature = "std")]
            Self::PathIsRelative(None) => f.write_str("relative path is not supported"),
            #[cfg(feature = "std")]
            Self::PathIsRelative(Some(path)) => {
                write!(f, "relative path is not supported: {}", path.display())
            },
            #[cfg(feature = "std")]
            Self::PathNotUnicode(None) => f.write_str("non-Unicode path is not supported"),
            #[cfg(feature = "std")]
            Self::PathNotUnicode(Some(path)) => {
                write!(f, "non-Unicode path is not supported: {}", path.display())
            },
            #[cfg(feature = "std")]
            Self::PathPrefixUnsupported(path) => {
                write!(f, "path prefix is not supported: {}", path.display())
            },
        }
    }
}

impl From<iri_string::validate::Error> for UriError {
    fn from(_: iri_string::validate::Error) -> Self {
        Self::Invalid(None)
    }
}

impl From<iri_string::types::CreationError<String>> for UriError {
    fn from(error: iri_string::types::CreationError<String>) -> Self {
        Self::Invalid(Some(error.into_source()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use iri_string::types::{UriStr, UriString};

    #[cfg(feature = "std")]
    #[test]
    fn filesystem_error_translation_preserves_variants_and_payloads() {
        use crate::IriError;

        assert!(matches!(
            UriError::from_iri(IriError::Invalid(None)),
            UriError::Invalid(None)
        ));
        let input = String::from("invalid input");
        let pointer = input.as_ptr();
        let UriError::Invalid(Some(input)) = UriError::from_iri(IriError::Invalid(Some(input)))
        else {
            panic!("expected an owned invalid-input error");
        };
        assert_eq!(input, "invalid input");
        assert_eq!(input.as_ptr(), pointer);

        for path in [None, Some(std::path::PathBuf::from("original/path"))] {
            assert!(
                matches!(UriError::from_iri(IriError::PathIsRelative(path.clone())),
                    UriError::PathIsRelative(original) if original == path)
            );
            assert!(
                matches!(UriError::from_iri(IriError::PathNotUnicode(path.clone())),
                    UriError::PathNotUnicode(original) if original == path)
            );
        }
        let path = std::path::PathBuf::from(r"\\?\C:\original");
        assert!(
            matches!(UriError::from_iri(IriError::PathPrefixUnsupported(path.clone())),
                UriError::PathPrefixUnsupported(original) if original == path)
        );
    }

    #[test]
    fn validation_errors_preserve_only_owned_input() {
        let input = "https://example.com/café";
        let borrowed = UriError::from(UriStr::new(input).unwrap_err());
        assert!(matches!(borrowed, UriError::Invalid(None)));
        assert_eq!(borrowed.to_string(), "invalid URI");
        let owned: UriResult<UriString> =
            UriString::try_from(String::from(input)).map_err(UriError::from);
        let error = owned.unwrap_err();
        assert!(matches!(&error, UriError::Invalid(Some(text)) if text == input));
        assert_eq!(error.to_string(), alloc::format!("invalid URI: {input}"));
        assert!(core::error::Error::source(&error).is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn path_errors_retain_payloads_and_format() {
        for (error, expected) in [
            (
                UriError::PathIsRelative(None),
                "relative path is not supported",
            ),
            (
                UriError::PathIsRelative(Some("relative".into())),
                "relative path is not supported: relative",
            ),
            (
                UriError::PathNotUnicode(None),
                "non-Unicode path is not supported",
            ),
            (
                UriError::PathNotUnicode(Some("path".into())),
                "non-Unicode path is not supported: path",
            ),
            (
                UriError::PathPrefixUnsupported("prefix".into()),
                "path prefix is not supported: prefix",
            ),
        ] {
            assert_eq!(error.clone().to_string(), expected);
        }
    }

    #[cfg(feature = "miette")]
    #[test]
    fn diagnostics_use_uri_codes() {
        use miette::Diagnostic;
        for (error, code) in [
            (UriError::Invalid(None), "dogma::uri::invalid"),
            (
                UriError::PathIsRelative(None),
                "dogma::uri_error::path_is_relative",
            ),
            (
                UriError::PathNotUnicode(None),
                "dogma::uri_error::path_not_unicode",
            ),
            (
                UriError::PathPrefixUnsupported("prefix".into()),
                "dogma::uri_error::path_prefix_unsupported",
            ),
        ] {
            assert_eq!(error.code().unwrap().to_string(), code);
            assert!(error.help().is_some());
        }
    }
}
