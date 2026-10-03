// This is free and unencumbered software released into the public domain.

use super::Iri;

pub type Uri<'a> = Iri<'a>; // TODO

// Remove the staging module and its dead-code allowance at URI activation.
#[allow(dead_code)]
pub(crate) mod staged {
    use crate::enums::uri_error::staged::{UriError, UriResult};
    use alloc::string::String;
    use core::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        str::FromStr,
    };
    use iri_string::types::{IriStr, UriStr, UriString};

    /// An ASCII URI stored as either a borrowed or owned validated string.
    ///
    /// Construction requires a scheme and rejects Unicode, malformed escapes,
    /// and relative references. Spelling is preserved without normalization.
    /// Equality, lexicographic ordering, and hashing use the exact string,
    /// independently of ownership.
    #[derive(Clone)]
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

        /// Converts this URI into an owned value with a `'static` lifetime.
        ///
        /// Borrowed strings are copied; owned strings move without allocating.
        pub fn into_owned(self) -> Uri<'static> {
            match self {
                Self::Borrowed(uri) => Uri::Owned(uri.into()),
                Self::Owned(uri) => Uri::Owned(uri),
            }
        }

        /// Clones this URI, preserving its spelling and ownership form.
        ///
        /// Borrowed strings remain borrowed; owned strings are copied.
        pub fn to_uri(&self) -> Uri<'_> {
            self.clone()
        }
    }

    impl AsRef<str> for Uri<'_> {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl Hash for Uri<'_> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_str().hash(state)
        }
    }

    impl PartialEq for Uri<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.as_str() == other.as_str()
        }
    }

    impl Eq for Uri<'_> {}

    impl PartialOrd for Uri<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Uri<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_str().cmp(other.as_str())
        }
    }

    impl fmt::Debug for Uri<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let variant = match self {
                Self::Borrowed(_) => "Uri::Borrowed",
                Self::Owned(_) => "Uri::Owned",
            };
            f.debug_tuple(variant).field(&self.as_str()).finish()
        }
    }

    impl fmt::Display for Uri<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.as_uri_str(), f)
        }
    }

    #[cfg(test)]
    mod tests {
        extern crate std;

        use super::*;
        use alloc::collections::BTreeSet;
        use core::hash::{BuildHasher, BuildHasherDefault};
        use std::collections::{hash_map::DefaultHasher, HashMap};

        fn representations(text: &str) -> [Uri<'_>; 2] {
            [Uri::try_from(text).unwrap(), text.parse().unwrap()]
        }

        #[test]
        fn formatting_preserves_spelling_and_identifies_variants() {
            let text = "HTTPS://EXAMPLE.com/%7e/../?q=%ff#";
            let [borrowed, owned] = representations(text);
            assert_eq!(alloc::format!("{borrowed}"), text);
            assert_eq!(alloc::format!("{owned}"), text);
            assert_eq!(
                alloc::format!("{borrowed:?}"),
                alloc::format!("Uri::Borrowed({text:?})")
            );
            assert_eq!(
                alloc::format!("{owned:?}"),
                alloc::format!("Uri::Owned({text:?})")
            );
        }

        #[test]
        fn value_traits_ignore_ownership() {
            let hasher = BuildHasherDefault::<DefaultHasher>::default();
            for text in ["https://example.com/", "urn:example:%C3%A9?#"] {
                let [borrowed, owned] = representations(text);
                assert_eq!(borrowed, owned);
                assert_eq!(owned, borrowed);
                assert_eq!(borrowed.partial_cmp(&owned), Some(Ordering::Equal));
                assert_eq!(owned.cmp(&borrowed), Ordering::Equal);
                assert_eq!(hasher.hash_one(&borrowed), hasher.hash_one(&owned));
                let mut values = HashMap::new();
                values.insert(borrowed, 1);
                assert_eq!(values.get(&owned), Some(&1));
                assert_eq!(values.insert(owned, 2), Some(1));
                assert_eq!(values.len(), 1);
            }
        }

        #[test]
        fn ordering_is_lexical_without_normalization() {
            for (lower, higher) in [
                ("https://example.com/a", "https://example.com/z"),
                ("HTTPS://example.com/", "https://example.com/"),
                ("https://example.com/%7E", "https://example.com/~"),
                ("https://example.com/%2F", "https://example.com/%2f"),
            ] {
                let mut values = BTreeSet::new();
                for left in representations(lower) {
                    for right in representations(higher) {
                        assert_ne!(left, right);
                        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
                        assert_eq!(right.partial_cmp(&left), Some(Ordering::Greater));
                        values.insert(right);
                    }
                    values.insert(left);
                }
                assert_eq!(values.len(), 2);
                assert_eq!(values.first().unwrap().as_str(), lower);
                assert_eq!(values.last().unwrap().as_str(), higher);
            }
        }

        #[test]
        fn into_owned_outlives_borrowed_input() {
            let text = "https://example.com/caf%C3%A9?lang=fr#top";
            let owned: Uri<'static> = {
                let input = String::from(text);
                Uri::try_from(input.as_str()).unwrap().into_owned()
            };
            assert!(matches!(owned, Uri::Owned(_)));
            assert_eq!(owned.as_str(), text);
        }

        #[test]
        fn into_owned_reuses_owned_allocation() {
            let input: Uri<'_> = "https://example.com/%2f".parse().unwrap();
            let pointer = input.as_str().as_ptr();
            let owned: Uri<'static> = input.into_owned();
            assert!(matches!(owned, Uri::Owned(_)));
            assert_eq!(owned.as_str(), "https://example.com/%2f");
            assert_eq!(owned.as_str().as_ptr(), pointer);
        }

        #[test]
        fn cloning_and_identity_conversion_preserve_ownership() {
            let text = "HTTPS://example.com/a/../%2f?#";
            let borrowed = Uri::try_from(text).unwrap();
            for copy in [borrowed.clone(), borrowed.to_uri()] {
                assert!(matches!(copy, Uri::Borrowed(_)));
                assert_eq!(copy.as_str(), text);
                assert_eq!(copy.as_str().as_ptr(), borrowed.as_str().as_ptr());
            }
            let owned: Uri<'_> = text.parse().unwrap();
            for copy in [owned.clone(), owned.to_uri()] {
                assert!(matches!(copy, Uri::Owned(_)));
                assert_eq!(copy.as_str(), text);
                assert_ne!(copy.as_str().as_ptr(), owned.as_str().as_ptr());
            }
            let cloned: Uri<'static> = owned.clone();
            drop(owned);
            assert_eq!(cloned.as_str(), text);
        }

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
