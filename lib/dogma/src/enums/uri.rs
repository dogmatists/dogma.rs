// This is free and unencumbered software released into the public domain.

use super::Iri;

pub type Uri<'a> = Iri<'a>; // TODO

// Remove the staging module and its dead-code allowance at URI activation.
#[allow(dead_code)]
pub(crate) mod staged {
    use crate::enums::{
        uri_error::staged::{UriError, UriResult},
        Iri, UriScheme,
    };
    use crate::structs::uri_authority::staged::UriAuthority;
    use alloc::string::String;
    use core::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        str::{FromStr, Split},
    };
    use iri_string::{
        components::AuthorityComponents,
        types::{IriStr, UriStr, UriString},
    };

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

    /// Converts a URI to an IRI without copying or changing its spelling.
    impl<'a> From<Uri<'a>> for Iri<'a> {
        fn from(uri: Uri<'a>) -> Self {
            match uri {
                Uri::Borrowed(uri) => Self::Borrowed(uri.as_ref()),
                Uri::Owned(uri) => Self::Owned(uri.into()),
            }
        }
    }

    /// Borrows either ownership form as an IRI without allocating.
    impl<'a> From<&'a Uri<'_>> for Iri<'a> {
        fn from(uri: &'a Uri<'_>) -> Self {
            Self::Borrowed(uri.as_iri_str())
        }
    }

    /// Borrows an ASCII IRI as a URI without allocating or encoding.
    ///
    /// Returns `UriError::Invalid(None)` for non-ASCII input, leaving the IRI
    /// unchanged. Percent-encoded non-ASCII data is accepted as written.
    impl<'a> TryFrom<&'a Iri<'_>> for Uri<'a> {
        type Error = UriError;

        fn try_from(iri: &'a Iri<'_>) -> UriResult<Self> {
            iri.as_iri_str()
                .as_uri()
                .map(Self::Borrowed)
                .ok_or(UriError::Invalid(None))
        }
    }

    /// Encodes an IRI as a URI, borrowing ASCII input and owning encoded output.
    ///
    /// Non-ASCII characters become UTF-8 percent escapes in every component,
    /// including hostnames; this does not perform IDNA/Punycode conversion.
    /// Existing escapes, case, and dot segments are preserved. The source is
    /// unchanged. Wire this adapter to `Iri::to_uri()` at URI activation.
    pub(crate) fn encode_iri<'a>(iri: &'a Iri<'_>) -> Uri<'a> {
        use iri_string::format::ToDedicatedString;

        let iri = iri.as_iri_str();
        match iri.as_uri() {
            Some(uri) => Uri::Borrowed(uri),
            None => Uri::Owned(iri.encode_to_uri().to_dedicated_string()),
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

        /// Returns the scheme, matching names case-insensitively.
        ///
        /// Unknown scheme names are lowercased. See [`Self::scheme_str`] for
        /// the original spelling.
        pub fn scheme(&self) -> UriScheme {
            let scheme = self.scheme_str();
            // `known-schemes` 0.2.0/0.2.1 parsing omits some listed variants.
            UriScheme::ALL
                .iter()
                .find(|known| known.as_str().eq_ignore_ascii_case(scheme))
                .cloned()
                .unwrap_or_else(|| UriScheme::Other(scheme.to_ascii_lowercase()))
        }

        /// Returns the scheme name with its original spelling.
        pub fn scheme_str(&self) -> &str {
            self.as_uri_str().scheme_str()
        }

        /// Reports whether an authority is present, even if empty.
        pub fn has_authority(&self) -> bool {
            self.authority_str().is_some()
        }

        /// Borrows the encoded authority, if present, directly from this URI.
        pub fn authority(&self) -> Option<UriAuthority<'_>> {
            UriAuthority::try_from(self).ok()
        }

        pub(crate) fn authority_components(&self) -> Option<AuthorityComponents<'_>> {
            self.as_uri_str().authority_components()
        }

        /// Returns the encoded authority, excluding its leading `//`.
        ///
        /// An empty authority is `Some("")`; an absent authority is `None`.
        pub fn authority_str(&self) -> Option<&str> {
            self.as_uri_str().authority_str()
        }

        /// Returns the encoded path without normalizing dot segments.
        pub fn path(&self) -> &str {
            self.as_uri_str().path_str()
        }

        /// Splits an absolute path after removing its first `/`.
        ///
        /// Preserves empty segments and escapes. Empty and rootless paths
        /// return `None`; the root path `/` yields one empty segment.
        pub fn path_segments(&self) -> Option<Split<'_, char>> {
            self.path().strip_prefix('/').map(|path| path.split('/'))
        }

        /// Reports whether a query is present, even if empty.
        pub fn has_query(&self) -> bool {
            self.query_str().is_some()
        }

        /// Returns the encoded query without `?`, distinguishing empty/absent.
        pub fn query_str(&self) -> Option<&str> {
            self.as_uri_str().query_str()
        }

        /// Reports whether a fragment is present, even if empty.
        pub fn has_fragment(&self) -> bool {
            self.fragment_str().is_some()
        }

        /// Returns the encoded fragment without `#`, distinguishing empty/absent.
        pub fn fragment_str(&self) -> Option<&str> {
            self.as_uri_str().fragment_str()
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
        fn into_iri_preserves_ownership_and_storage() {
            let input = String::from("HTTPS://EXAMPLE.com/a/../%C3%A9?#");
            let iri = {
                let uri = Uri::try_from(input.as_str()).unwrap();
                Iri::from(uri)
            };
            assert!(matches!(iri, Iri::Borrowed(_)));
            assert_eq!(iri.as_str(), input);
            assert_eq!(iri.as_str().as_ptr(), input.as_ptr());

            let owned: Iri<'static> = {
                let uri: Uri<'static> = input.parse().unwrap();
                let pointer = uri.as_str().as_ptr();
                let iri = Iri::from(uri);
                assert!(matches!(iri, Iri::Owned(_)));
                assert_eq!(iri.as_str().as_ptr(), pointer);
                iri
            };
            drop(input);
            assert_eq!(owned.as_str(), "HTTPS://EXAMPLE.com/a/../%C3%A9?#");
        }

        #[test]
        fn iri_views_borrow_both_uri_ownership_forms() {
            for uri in representations("https://example.com/%2f") {
                let iri = Iri::from(&uri);
                assert!(matches!(iri, Iri::Borrowed(_)));
                assert_eq!(iri.as_str(), uri.as_str());
                assert_eq!(iri.as_str().as_ptr(), uri.as_str().as_ptr());
            }
        }

        #[test]
        fn checked_iri_borrowing_preserves_storage_and_spelling() {
            for text in ["HTTPS://EXAMPLE.com/a/../%c3%a9?#", "urn:example:%FF"] {
                for iri in [Iri::try_from(text).unwrap(), text.parse().unwrap()] {
                    let uri = Uri::try_from(&iri).unwrap();
                    assert!(matches!(uri, Uri::Borrowed(_)));
                    assert_eq!(uri.as_str(), text);
                    assert_eq!(uri.as_str().as_ptr(), iri.as_str().as_ptr());
                    assert_eq!(iri.as_str(), text);
                }
            }
        }

        #[test]
        fn checked_iri_borrowing_rejects_unicode_without_encoding() {
            for text in [
                "https://usér@example.com/",
                "https://例.example/",
                "https://example.com/café",
                "https://example.com/?q=é",
                "https://example.com/#é",
            ] {
                for iri in [Iri::try_from(text).unwrap(), text.parse().unwrap()] {
                    let pointer = iri.as_str().as_ptr();
                    assert!(matches!(Uri::try_from(&iri), Err(UriError::Invalid(None))));
                    assert_eq!(iri.as_str(), text);
                    assert_eq!(iri.as_str().as_ptr(), pointer);
                }
            }
        }

        #[test]
        fn encoding_borrows_ascii_from_both_ownership_forms() {
            for text in [
                "HTTPS://EXAMPLE.com/a/../%c3%a9?x=%FF+#",
                "urn:example:value",
            ] {
                for iri in [Iri::try_from(text).unwrap(), text.parse().unwrap()] {
                    let uri = encode_iri(&iri);
                    assert!(matches!(uri, Uri::Borrowed(_)));
                    assert_eq!(uri.as_str(), text);
                    assert_eq!(uri.as_str().as_ptr(), iri.as_str().as_ptr());
                    assert_eq!(iri.as_str(), text);
                }
            }
        }

        #[test]
        fn encoding_escapes_unicode_in_every_component() {
            for (text, expected) in [
                (
                    "https://usér:páss@example.com/",
                    "https://us%C3%A9r:p%C3%A1ss@example.com/",
                ),
                ("https://例.example/", "https://%E4%BE%8B.example/"),
                ("https://example.com/café", "https://example.com/caf%C3%A9"),
                ("https://example.com/?q=é", "https://example.com/?q=%C3%A9"),
                ("https://example.com/#é", "https://example.com/#%C3%A9"),
                ("x:/😀?q=\u{e000}", "x:/%F0%9F%98%80?q=%EE%80%80"),
                (
                    "HTTPS://é@例.example/a/../%c3%a9/é?q=%FF+é#%2fé",
                    "HTTPS://%C3%A9@%E4%BE%8B.example/a/../%c3%a9/%C3%A9?q=%FF+%C3%A9#%2f%C3%A9",
                ),
            ] {
                for iri in [Iri::try_from(text).unwrap(), text.parse().unwrap()] {
                    let pointer = iri.as_str().as_ptr();
                    let uri = encode_iri(&iri);
                    assert!(matches!(uri, Uri::Owned(_)));
                    assert_eq!(uri.as_str(), expected);
                    assert!(UriStr::new(uri.as_str()).is_ok());
                    assert_eq!(iri.as_str(), text);
                    assert_eq!(iri.as_str().as_ptr(), pointer);
                }
            }
        }

        #[test]
        fn schemes_recognize_all_known_names_case_insensitively() {
            for expected in UriScheme::ALL {
                for name in [
                    String::from(expected.as_str()),
                    expected.as_str().to_ascii_uppercase(),
                ] {
                    let text = alloc::format!("{name}:value");
                    for uri in representations(&text) {
                        assert_eq!(uri.scheme(), *expected, "{text}");
                        assert_eq!(uri.scheme_str(), name);
                        assert_eq!(uri.as_str(), text);
                    }
                }
            }
            for uri in representations("X-Example+V1.2:Payload") {
                assert_eq!(
                    uri.scheme(),
                    UriScheme::Other(String::from("x-example+v1.2"))
                );
                assert_eq!(uri.scheme_str(), "X-Example+V1.2");
            }
            for uri in representations("hTtPs://example.com/") {
                assert_eq!(uri.scheme(), UriScheme::Https);
                assert_eq!(uri.scheme_str(), "hTtPs");
            }
        }

        #[test]
        fn components_preserve_encoded_spelling() {
            let text = "HTTPS://u%73er:p%40ss@EXAMPLE.com:443/a%2fb/../c?x=%ff+#P%61rt";
            for uri in representations(text) {
                assert_eq!(uri.authority_str(), Some("u%73er:p%40ss@EXAMPLE.com:443"));
                let authority = uri.authority_components().unwrap();
                assert_eq!(authority.userinfo(), Some("u%73er:p%40ss"));
                assert_eq!(authority.host(), "EXAMPLE.com");
                assert_eq!(authority.port(), Some("443"));
                assert_eq!(uri.path(), "/a%2fb/../c");
                assert_eq!(uri.query_str(), Some("x=%ff+"));
                assert_eq!(uri.fragment_str(), Some("P%61rt"));
                assert_eq!(uri.as_str(), text);
                assert!(uri.has_authority() && uri.has_query() && uri.has_fragment());
            }
        }

        #[test]
        fn components_distinguish_empty_and_absent() {
            for (text, authority, query, fragment) in [
                ("x:", None, None, None),
                ("x://", Some(""), None, None),
                ("x:?", None, Some(""), None),
                ("x:#", None, None, Some("")),
                ("x://?#", Some(""), Some(""), Some("")),
            ] {
                for uri in representations(text) {
                    assert_eq!(uri.authority_str(), authority);
                    assert_eq!(uri.has_authority(), authority.is_some());
                    assert_eq!(uri.authority_components().is_some(), authority.is_some());
                    assert_eq!(uri.path(), "");
                    assert_eq!(uri.query_str(), query);
                    assert_eq!(uri.has_query(), query.is_some());
                    assert_eq!(uri.fragment_str(), fragment);
                    assert_eq!(uri.has_fragment(), fragment.is_some());
                }
            }
        }

        #[test]
        fn path_segments_preserve_empty_encoded_and_dot_segments() {
            for (text, expected) in [
                ("x:", None),
                ("urn:example:value", None),
                ("x:/", Some(alloc::vec![""])),
                ("x:/a//b/", Some(alloc::vec!["a", "", "b", ""])),
                ("x:/a%2Fb/./../", Some(alloc::vec!["a%2Fb", ".", "..", ""])),
            ] {
                for uri in representations(text) {
                    assert_eq!(
                        uri.path_segments()
                            .map(|parts| parts.collect::<alloc::vec::Vec<_>>()),
                        expected,
                        "{text}"
                    );
                }
            }
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
