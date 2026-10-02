// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "uri")]
use crate::enums::Uri;
use crate::{
    enums::{IriError, IriScheme},
    prelude::{fmt, str::Split, FromStr, String},
    structs::IriAuthority,
};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use iri_string::{
    components::AuthorityComponents,
    types::{IriStr, IriString},
};

/// An IRI stored as either a borrowed or owned string.
///
/// Equality, ordering, and hashing use the exact IRI string, independently of
/// ownership. Ordering is lexicographic; no normalization is performed.
#[derive(Clone)]
pub enum Iri<'a> {
    Borrowed(&'a IriStr),
    Owned(IriString),
}

impl<'a> FromStr for Iri<'a> {
    type Err = IriError;

    fn from_str(iri_str: &str) -> Result<Self, Self::Err> {
        IriStr::new(iri_str)
            .map(|iri_str| Iri::Owned(iri_str.into()))
            .map_err(|error| error.into())
    }
}

impl<'a> From<&'a IriStr> for Iri<'a> {
    fn from(iri_str: &'a IriStr) -> Self {
        Iri::Borrowed(iri_str)
    }
}

impl<'a> From<&'a IriString> for Iri<'a> {
    fn from(iri_string: &'a IriString) -> Self {
        Iri::Borrowed(iri_string.as_ref())
    }
}

impl From<IriString> for Iri<'static> {
    fn from(iri_string: IriString) -> Self {
        Iri::Owned(iri_string)
    }
}

impl<'a> TryFrom<&'a str> for Iri<'a> {
    type Error = IriError;

    fn try_from(iri_str: &'a str) -> Result<Self, Self::Error> {
        IriStr::new(iri_str)
            .map(|iri_str| Iri::Borrowed(iri_str))
            .map_err(|error| error.into())
    }
}

impl TryFrom<String> for Iri<'static> {
    type Error = IriError;

    fn try_from(iri_string: String) -> Result<Self, Self::Error> {
        IriString::try_from(iri_string)
            .map(|iri_string| Iri::Owned(iri_string))
            .map_err(|error| error.into())
    }
}

/// Converts an absolute Unicode filesystem path into an owned file IRI.
///
/// On non-Windows platforms, uses an empty authority and percent-encodes path
/// data, preserving separators and IRI-compatible Unicode. Relative and
/// non-Unicode paths are rejected.
#[cfg(feature = "std")]
impl TryFrom<&std::path::Path> for Iri<'static> {
    type Error = IriError;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        if !path.is_absolute() {
            return Err(IriError::PathIsRelative(Some(path.into())));
        }
        let Some(path) = path.to_str() else {
            return Err(IriError::PathNotUnicode(Some(path.into())));
        };
        #[cfg(not(windows))]
        let iri_string = alloc::format!(
            "file://{}",
            iri_string::percent_encode::PercentEncodedForIri::from_path(path)
        );
        #[cfg(windows)]
        let iri_string = alloc::format!("file:{}", path);
        Self::try_from(iri_string)
    }
}

impl Iri<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            Iri::Borrowed(iri) => iri.as_str(),
            Iri::Owned(iri) => iri.as_str(),
        }
    }

    /// Converts this IRI into an owned value with a `'static` lifetime.
    ///
    /// Borrowed strings are copied; owned strings are moved without allocating.
    pub fn into_owned(self) -> Iri<'static> {
        match self {
            Iri::Borrowed(iri) => Iri::Owned(iri.into()),
            Iri::Owned(iri) => Iri::Owned(iri),
        }
    }

    /// Returns the scheme, matching names case-insensitively.
    ///
    /// Unrecognized schemes have lowercase names. Use [`Self::scheme_str`] for
    /// the original spelling.
    pub fn scheme(&self) -> IriScheme {
        let scheme = self.scheme_str();
        // `known-schemes` 0.2.0/0.2.1 parsing omits some listed variants.
        IriScheme::ALL
            .iter()
            .find(|known| known.as_str().eq_ignore_ascii_case(scheme))
            .cloned()
            .unwrap_or_else(|| IriScheme::Other(scheme.to_ascii_lowercase()))
    }

    /// Returns the scheme name with its original spelling.
    pub fn scheme_str(&self) -> &str {
        match self {
            Iri::Borrowed(iri) => iri.scheme_str(),
            Iri::Owned(iri) => iri.scheme_str(),
        }
    }

    pub fn has_authority(&self) -> bool {
        self.authority_str().is_some()
    }

    pub fn authority(&self) -> Option<IriAuthority<'_>> {
        IriAuthority::try_from(self).ok()
    }

    pub(crate) fn authority_components(&self) -> Option<AuthorityComponents<'_>> {
        match self {
            Iri::Borrowed(iri) => iri.authority_components(),
            Iri::Owned(iri) => iri.authority_components(),
        }
    }

    pub fn authority_str(&self) -> Option<&str> {
        match self {
            Iri::Borrowed(iri) => iri.authority_str(),
            Iri::Owned(iri) => iri.authority_str(),
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Iri::Borrowed(iri) => iri.path_str(),
            Iri::Owned(iri) => iri.path_str(),
        }
    }

    pub fn path_segments(&self) -> Option<Split<'_, char>> {
        let path = self.path();
        path.strip_prefix('/').map(|remainder| remainder.split('/'))
    }

    pub fn has_query(&self) -> bool {
        self.query_str().is_some()
    }

    pub fn query_str(&self) -> Option<&str> {
        match self {
            Iri::Borrowed(iri) => iri.query_str(),
            Iri::Owned(iri) => iri.query_str(),
        }
    }

    pub fn has_fragment(&self) -> bool {
        self.fragment_str().is_some()
    }

    pub fn fragment_str(&self) -> Option<&str> {
        match self {
            Iri::Borrowed(iri) => iri.fragment_str(),
            Iri::Owned(iri) => iri.fragment_str(),
        }
    }

    #[cfg(feature = "uri")]
    pub fn to_uri(&self) -> Uri<'_> {
        self.clone() // TODO
    }

    #[cfg(feature = "std")]
    pub fn to_path(&self) -> Option<std::path::PathBuf> {
        if self.scheme() != IriScheme::File {
            return None;
        }
        Some(std::path::PathBuf::from(self.path()))
    }
}

impl Hash for Iri<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl PartialEq for Iri<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Iri<'_> {}

impl PartialOrd for Iri<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Iri<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl fmt::Debug for Iri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Iri::Borrowed(value) => f
                .debug_tuple("Iri::Borrowed")
                .field(&value.as_str())
                .finish(),
            Iri::Owned(value) => f //
                .debug_tuple("Iri::Owned")
                .field(&value.as_str())
                .finish(),
        }
    }
}

impl fmt::Display for Iri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Iri::Borrowed(iri) => iri.fmt(f),
            Iri::Owned(iri) => iri.fmt(f),
        }
    }
}

#[cfg(feature = "clap")]
include!("integrations/clap.rs");

#[cfg(test)]
mod tests {
    extern crate std;

    use super::{Iri, IriScheme};
    use alloc::{collections::BTreeSet, string::String};
    use core::{
        cmp::Ordering,
        hash::{BuildHasher, BuildHasherDefault},
    };
    use std::collections::{hash_map::DefaultHasher, HashMap};

    fn representations(text: &str) -> [Iri<'_>; 2] {
        [Iri::try_from(text).unwrap(), text.parse().unwrap()]
    }

    #[cfg(all(feature = "std", not(windows)))]
    #[test]
    fn from_path_encodes_posix_path_data() {
        for (input, expected) in [
            ("/tmp/a b", "/tmp/a%20b"),
            ("/tmp/a#b", "/tmp/a%23b"),
            ("/tmp/a?b", "/tmp/a%3Fb"),
            ("/tmp/a%20b", "/tmp/a%2520b"),
            ("/tmp/%", "/tmp/%25"),
            (r"/tmp/a\b", "/tmp/a%5Cb"),
            ("/tmp/a\nb", "/tmp/a%0Ab"),
            ("/tmp/café/東京", "/tmp/café/東京"),
            ("/tmp/\u{e000}", "/tmp/%EE%80%80"),
            ("/", "/"),
            ("/tmp/./dir/../file", "/tmp/./dir/../file"),
            ("//server/share", "//server/share"),
            ("///tmp/file", "///tmp/file"),
        ] {
            let iri = Iri::try_from(std::path::Path::new(input)).unwrap();
            assert_eq!(iri.scheme(), IriScheme::File, "{input:?}");
            assert_eq!(iri.authority_str(), Some(""), "{input:?}");
            assert_eq!(iri.path(), expected, "{input:?}");
            assert!(!iri.has_query(), "{input:?}");
            assert!(!iri.has_fragment(), "{input:?}");
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_path_rejects_relative_paths() {
        for input in [
            "",
            "relative/file",
            "../a b#c?d%",
            r"C:relative\file",
            r"\rooted",
        ] {
            let path = std::path::Path::new(input);
            assert!(matches!(
                Iri::try_from(path),
                Err(crate::IriError::PathIsRelative(Some(original)))
                    if original.as_path() == path
            ));
        }
    }

    #[cfg(all(feature = "std", unix))]
    #[test]
    fn from_path_rejects_non_unicode_paths() {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt, path::Path};

        let path = Path::new(OsStr::from_bytes(b"/tmp/\xff"));
        assert!(matches!(
            Iri::try_from(path),
            Err(crate::IriError::PathNotUnicode(Some(original)))
                if original.as_path() == path
        ));
    }

    #[test]
    fn scheme_recognizes_telnet_and_tftp() {
        let schemes = ["telnet://127.0.0.1/", "tftp://127.0.0.1/"]
            .map(|text| Iri::try_from(text).unwrap().scheme());
        assert_eq!(schemes, [IriScheme::Telnet, IriScheme::Tftp]);
        assert_eq!(schemes.map(|scheme| scheme.to_port()), [Some(23), Some(69)]);
    }

    #[test]
    fn scheme_recognizes_all_known_names() {
        for expected in IriScheme::ALL {
            for name in [
                String::from(expected.as_str()),
                expected.as_str().to_ascii_uppercase(),
            ] {
                let text = alloc::format!("{name}:value");
                for iri in representations(&text) {
                    assert_eq!(iri.scheme(), *expected, "{text}");
                }
            }
        }
    }

    #[test]
    fn scheme_recognition_ignores_ascii_case() {
        for (text, spelling, expected) in [
            (
                "https://example.com/Case?Q=Value#Part",
                "https",
                IriScheme::Https,
            ),
            (
                "HTTPS://example.com/Case?Q=Value#Part",
                "HTTPS",
                IriScheme::Https,
            ),
            (
                "hTtPs://example.com/Case?Q=Value#Part",
                "hTtPs",
                IriScheme::Https,
            ),
            ("HTTP://example.com/", "HTTP", IriScheme::Http),
            ("FiLe:/Example", "FiLe", IriScheme::File),
        ] {
            for iri in representations(text) {
                assert_eq!(iri.scheme(), expected);
                assert_eq!(iri.scheme_str(), spelling);
                assert_eq!(iri.as_str(), text);
            }
        }
    }

    #[test]
    fn scheme_normalizes_unknown_names() {
        let expected = IriScheme::Other(String::from("x-example+v1.2"));
        for (text, spelling) in [
            ("x-example+v1.2:Payload", "x-example+v1.2"),
            ("X-Example+V1.2:Payload", "X-Example+V1.2"),
        ] {
            for iri in representations(text) {
                assert_eq!(iri.scheme(), expected);
                assert_eq!(iri.scheme_str(), spelling);
                assert_eq!(iri.as_str(), text);
            }
        }
    }

    #[test]
    fn into_owned_outlives_borrowed_input() {
        let text = "https://example.com/café?lang=fr#top";
        let owned: Iri<'static> = {
            let input = String::from(text);
            Iri::try_from(input.as_str()).unwrap().into_owned()
        };
        assert!(matches!(&owned, Iri::Owned(_)));
        assert_eq!(owned.as_str(), text);
    }

    #[test]
    fn into_owned_reuses_owned_string() {
        let text = "https://example.com/café?lang=fr#top";
        let input: Iri<'_> = text.parse().unwrap();
        let pointer = input.as_str().as_ptr();
        let owned: Iri<'static> = input.into_owned();
        assert!(matches!(&owned, Iri::Owned(_)));
        assert_eq!(owned.as_str(), text);
        assert_eq!(owned.as_str().as_ptr(), pointer);
    }

    #[test]
    fn equality_ignores_ownership() {
        for text in [
            "https://example.com/",
            "https://example.com/café?lang=fr#top",
        ] {
            let [borrowed, owned] = representations(text);
            assert_eq!(borrowed, owned);
            assert_eq!(owned, borrowed);
            assert_eq!(borrowed.partial_cmp(&owned), Some(Ordering::Equal));
            assert_eq!(owned.partial_cmp(&borrowed), Some(Ordering::Equal));
        }
    }

    #[test]
    fn hashing_ignores_ownership() {
        let hasher = BuildHasherDefault::<DefaultHasher>::default();
        let [borrowed, owned] = representations("https://example.com/café");
        assert_eq!(hasher.hash_one(&borrowed), hasher.hash_one(&owned));
    }

    #[test]
    fn hash_map_keys_ignore_ownership() {
        let [borrowed, owned] = representations("https://example.com/café");
        let mut values = HashMap::new();
        values.insert(borrowed, 1);
        assert_eq!(values.get(&owned), Some(&1));
        assert_eq!(values.insert(owned, 2), Some(1));
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn ordered_set_keys_ignore_ownership() {
        let lower = "https://example.com/a";
        let higher = "https://example.com/café";
        let mut values = BTreeSet::new();
        for text in [higher, lower] {
            for iri in representations(text) {
                values.insert(iri);
            }
        }
        assert_eq!(values.len(), 2);
        assert_eq!(values.first().unwrap().as_str(), lower);
        assert_eq!(values.last().unwrap().as_str(), higher);
    }

    #[test]
    fn ordering_is_lexical_for_all_ownership_combinations() {
        for (lower, higher) in [
            ("https://example.com/a", "https://example.com/z"),
            ("https://example.com/a", "https://example.com/é"),
            ("HTTPS://example.com/", "https://example.com/"),
            ("https://example.com/%7E", "https://example.com/~"),
        ] {
            for left in representations(lower) {
                for right in representations(higher) {
                    assert_ne!(left, right);
                    assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
                    assert_eq!(right.partial_cmp(&left), Some(Ordering::Greater));
                }
            }
        }
    }
}
