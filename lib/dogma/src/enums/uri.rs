// This is free and unencumbered software released into the public domain.

use super::Iri;

pub type Uri<'a> = Iri<'a>; // TODO

// Remove the staging module and its dead-code allowance at URI activation.
#[allow(dead_code)]
pub(crate) mod staged {
    use crate::enums::uri_error::staged::{UriError, UriResult};
    use alloc::string::String;
    use core::str::FromStr;
    use iri_string::types::{IriStr, UriStr, UriString};

    /// An ASCII URI stored as either a borrowed or owned validated string.
    ///
    /// Construction requires a scheme and rejects Unicode, malformed escapes,
    /// and relative references. Spelling is preserved without normalization.
    pub enum Uri<'a> {
        Borrowed(&'a UriStr),
        Owned(UriString),
    }

    impl FromStr for Uri<'_> {
        type Err = UriError;

        fn from_str(text: &str) -> UriResult<Self> {
            UriStr::new(text)
                .map(|uri| Self::Owned(uri.into()))
                .map_err(Into::into)
        }
    }

    impl<'a> TryFrom<&'a str> for Uri<'a> {
        type Error = UriError;

        fn try_from(text: &'a str) -> UriResult<Self> {
            UriStr::new(text).map(Self::Borrowed).map_err(Into::into)
        }
    }

    impl TryFrom<String> for Uri<'static> {
        type Error = UriError;

        fn try_from(text: String) -> UriResult<Self> {
            UriString::try_from(text)
                .map(Self::Owned)
                .map_err(Into::into)
        }
    }

    impl<'a> From<&'a UriStr> for Uri<'a> {
        fn from(uri: &'a UriStr) -> Self {
            Self::Borrowed(uri)
        }
    }

    impl<'a> From<&'a UriString> for Uri<'a> {
        fn from(uri: &'a UriString) -> Self {
            Self::Borrowed(uri.as_ref())
        }
    }

    impl From<UriString> for Uri<'static> {
        fn from(uri: UriString) -> Self {
            Self::Owned(uri)
        }
    }

    impl Uri<'_> {
        /// Borrows the validated URI without allocating.
        pub fn as_uri_str(&self) -> &UriStr {
            match self {
                Self::Borrowed(uri) => uri,
                Self::Owned(uri) => uri.as_ref(),
            }
        }

        /// Borrows the URI as an IRI; every URI is also a valid IRI.
        pub fn as_iri_str(&self) -> &IriStr {
            self.as_uri_str().as_ref()
        }

        /// Returns the original string without normalization or decoding.
        pub fn as_str(&self) -> &str {
            self.as_uri_str().as_str()
        }
    }

    impl AsRef<str> for Uri<'_> {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn strict_constructors_reject_invalid_uris() {
            for text in [
                "",
                "/relative",
                "//example.com/path",
                "1scheme:value",
                "https://usér@example.com/",
                "https://例.example/",
                "https://example.com/café",
                "https://example.com/?q=é",
                "https://example.com/#é",
                "https://example.com/%",
                "https://example.com/%2",
                "https://example.com/%GG",
                "https://example.com/a b",
            ] {
                assert!(
                    matches!(Uri::try_from(text), Err(UriError::Invalid(None))),
                    "{text}"
                );
                assert!(
                    matches!(text.parse::<Uri<'_>>(), Err(UriError::Invalid(None))),
                    "{text}"
                );
                assert!(
                    matches!(Uri::try_from(String::from(text)),
                    Err(UriError::Invalid(Some(original))) if original == text),
                    "{text}"
                );
            }
        }

        #[test]
        fn constructors_preserve_spelling_and_borrowed_views() {
            for text in [
                "https://user:pass@example.com:443/a%2fb/../c?x=%FF#Part",
                "HTTPS://EXAMPLE.com/",
                "urn:example:value",
                "x:",
                "x://?#",
            ] {
                let borrowed = Uri::try_from(text).unwrap();
                assert!(matches!(borrowed, Uri::Borrowed(_)));
                assert_eq!(borrowed.as_str().as_ptr(), text.as_ptr());
                let parsed: Uri<'_> = text.parse().unwrap();
                assert!(matches!(parsed, Uri::Owned(_)));
                for uri in [borrowed, parsed] {
                    assert_eq!(uri.as_str(), text);
                    assert_eq!(uri.as_ref(), text);
                    assert_eq!(uri.as_uri_str().as_str().as_ptr(), uri.as_str().as_ptr());
                    assert_eq!(uri.as_iri_str().as_str().as_ptr(), uri.as_str().as_ptr());
                }
            }
        }

        #[test]
        fn upstream_constructors_preserve_storage() {
            let text = "https://example.com/%C3%A9";
            let upstream = UriStr::new(text).unwrap();
            let borrowed = Uri::from(upstream);
            assert!(matches!(borrowed, Uri::Borrowed(_)));
            assert_eq!(borrowed.as_str().as_ptr(), text.as_ptr());

            let upstream = UriString::try_from(String::from(text)).unwrap();
            let pointer = upstream.as_str().as_ptr();
            let borrowed = Uri::from(&upstream);
            assert!(matches!(borrowed, Uri::Borrowed(_)));
            assert_eq!(borrowed.as_str().as_ptr(), pointer);
            let moved: Uri<'static> = Uri::from(upstream);
            assert!(matches!(moved, Uri::Owned(_)));
            assert_eq!(moved.as_str().as_ptr(), pointer);

            let input = String::from(text);
            let pointer = input.as_ptr();
            let moved: Uri<'static> = Uri::try_from(input).unwrap();
            assert!(matches!(moved, Uri::Owned(_)));
            assert_eq!(moved.as_str().as_ptr(), pointer);
        }
    }
}
