// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use crate::enums::IriToPathError;
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
/// On Windows, ordinary drive paths use an empty authority, forward slashes,
/// and percent-encoded path data. UNC paths use the server as a percent-encoded
/// registered-name authority, with the share as the first path segment.
/// Windows verbatim and device namespace prefixes are rejected explicitly.
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
        let iri_string = {
            use iri_string::percent_encode::PercentEncodedForIri;
            use std::path::{Component, Path, Prefix};

            let Some(Component::Prefix(prefix)) = Path::new(path).components().next() else {
                return Err(IriError::PathPrefixUnsupported(path.into()));
            };
            match prefix.kind() {
                Prefix::Disk(_) => {
                    let path = path.replace('\\', "/");
                    alloc::format!("file:///{}", PercentEncodedForIri::from_path(&path))
                }
                Prefix::UNC(server, share) => {
                    let server = server
                        .to_str()
                        .ok_or_else(|| IriError::PathNotUnicode(Some(path.into())))?;
                    let share = share
                        .to_str()
                        .ok_or_else(|| IriError::PathNotUnicode(Some(path.into())))?;
                    // Slice the original path to preserve dot segments and
                    // trailing separators instead of rebuilding components.
                    let suffix = path[prefix.as_os_str().len()..].replace('\\', "/");
                    alloc::format!(
                        "file://{}/{}{}",
                        PercentEncodedForIri::from_reg_name(server),
                        PercentEncodedForIri::from_path_segment(share),
                        PercentEncodedForIri::from_path(&suffix)
                    )
                }
                _ => return Err(IriError::PathPrefixUnsupported(path.into())),
            }
        };
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

    /// Returns the percent-decoded path component of a file IRI.
    ///
    /// On non-Windows platforms, only absent or empty authorities and bare
    /// `localhost` (ignoring ASCII case) are accepted.
    ///
    /// The IRI path must start with a literal `/`; empty or rootless paths
    /// return `None`. For local Windows drive paths, the decoded path must start
    /// with `/C:/` (using any ASCII drive letter); the leading slash is removed
    /// and path separators become backslashes.
    ///
    /// On Windows, a nonempty registered-name or IPv4 host and a share path
    /// produce a UNC path. Hostnames are percent-decoded as UTF-8; `localhost`
    /// (ignoring ASCII case after decoding) with a drive path denotes that local
    /// drive, and otherwise denotes a UNC server. User information, ports,
    /// bracketed IP literals, and the server names `.`, `..`, and `?` are
    /// unsupported. Shares must be nonempty, must not be `.` or `..`, and must
    /// not contain `:`.
    ///
    /// Returns `None` if a query or fragment is present, even if it is empty.
    /// Literal `?` and `#` in paths must be encoded as `%3F` and `%23`.
    ///
    /// Escapes are decoded exactly once as UTF-8; `+` remains literal.
    /// Returns `None` for other schemes, invalid UTF-8, NUL bytes, or encoded
    /// native separators (`/`, and on Windows also `\`) in paths or hostnames.
    ///
    /// See [`Self::try_to_path`] for error details.
    #[cfg(feature = "std")]
    pub fn to_path(&self) -> Option<std::path::PathBuf> {
        self.try_to_path().ok()
    }

    /// Converts a file IRI's path using the rules of [`Self::to_path`].
    ///
    /// # Errors
    ///
    /// Returns an [`IriToPathError`] describing the unsupported component or
    /// invalid path data that prevents conversion.
    #[cfg(feature = "std")]
    pub fn try_to_path(&self) -> Result<std::path::PathBuf, IriToPathError> {
        if self.scheme() != IriScheme::File {
            return Err(IriToPathError::UnsupportedScheme);
        }
        if self.has_query() {
            return Err(IriToPathError::UnsupportedQuery);
        }
        if self.has_fragment() {
            return Err(IriToPathError::UnsupportedFragment);
        }
        #[cfg(not(windows))]
        if self.authority_str().is_some_and(|authority| {
            !authority.is_empty() && !authority.eq_ignore_ascii_case("localhost")
        }) {
            return Err(IriToPathError::UnsupportedAuthority);
        }
        #[cfg(windows)]
        let host = self
            .authority_components()
            .map(|authority| {
                if authority.userinfo().is_some()
                    || authority.port().is_some()
                    || authority.host().starts_with('[')
                {
                    return Err(IriToPathError::UnsupportedAuthority);
                }
                let host = decode_file_path_data(authority.host())?;
                // These names would select a device namespace or a dot segment.
                if matches!(host.as_str(), "." | ".." | "?") {
                    return Err(IriToPathError::UnsupportedAuthority);
                }
                Ok(host)
            })
            .transpose()?;
        let path = self.path();
        if !path.starts_with('/') {
            return Err(IriToPathError::PathNotAbsolute);
        }
        let decoded = decode_file_path_data(path)?;
        #[cfg(windows)]
        let decoded = {
            let drive_path = matches!(
                decoded.as_bytes(),
                [b'/', drive, b':', b'/', ..] if drive.is_ascii_alphabetic()
            );
            match host.as_deref().filter(|host| !host.is_empty()) {
                Some(host) if !(host.eq_ignore_ascii_case("localhost") && drive_path) => {
                    let share = decoded[1..].split('/').next().unwrap_or("");
                    if matches!(share, "" | "." | "..") || share.contains(':') {
                        return Err(IriToPathError::InvalidUncShare);
                    }
                    alloc::format!("\\\\{}{}", host, decoded.replace('/', "\\"))
                }
                _ => {
                    // A rooted path alone depends on the current drive.
                    if !drive_path {
                        return Err(IriToPathError::PathNotAbsolute);
                    }
                    decoded[1..].replace('/', "\\")
                }
            }
        };
        Ok(std::path::PathBuf::from(decoded))
    }
}

// Shared decoding for paths and Windows UNC hostnames.
#[cfg(feature = "std")]
fn decode_file_path_data(text: &str) -> Result<String, IriToPathError> {
    let mut decoded = alloc::vec::Vec::with_capacity(text.len());
    let mut bytes = text.bytes();
    while let Some(mut byte) = bytes.next() {
        if byte == b'%' {
            let high = char::from(bytes.next().ok_or(IriToPathError::InvalidEncoding)?)
                .to_digit(16)
                .ok_or(IriToPathError::InvalidEncoding)?;
            let low = char::from(bytes.next().ok_or(IriToPathError::InvalidEncoding)?)
                .to_digit(16)
                .ok_or(IriToPathError::InvalidEncoding)?;
            byte = ((high << 4) | low) as u8;
            if byte == 0 {
                return Err(IriToPathError::NulByte);
            }
            if std::path::is_separator(char::from(byte)) {
                return Err(IriToPathError::EncodedSeparator);
            }
        }
        decoded.push(byte);
    }
    String::from_utf8(decoded).map_err(|_| IriToPathError::InvalidEncoding)
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

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn from_path_encodes_windows_drive_paths() {
        for (input, expected) in [
            (r"C:\", "file:///C:/"),
            ("c:/Temp/file", "file:///c:/Temp/file"),
            (r"C:\Temp\a b#c%d", "file:///C:/Temp/a%20b%23c%25d"),
            (r"C:\Temp\%20", "file:///C:/Temp/%2520"),
            (r"C:\café\東京", "file:///C:/café/東京"),
            (r"D:\dir/./sub\..\file", "file:///D:/dir/./sub/../file"),
        ] {
            let iri = Iri::try_from(std::path::Path::new(input)).unwrap();
            assert_eq!(iri.as_str(), expected, "{input}");
            assert_eq!(iri.authority_str(), Some(""), "{input}");
            assert!(!iri.has_query(), "{input}");
            assert!(!iri.has_fragment(), "{input}");
            let decoded = iri.try_to_path().unwrap();
            assert!(decoded.is_absolute(), "{input}");
            assert_eq!(
                decoded.as_os_str(),
                std::ffi::OsStr::new(&input.replace('/', "\\")),
                "{input}"
            );
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn to_path_decodes_windows_drive_paths() {
        for (input, expected) in [
            ("file:///C:/", r"C:\"),
            ("file:/c:/Temp/file", r"c:\Temp\file"),
            ("file://localhost/C:/Temp/a%20b", r"C:\Temp\a b"),
            ("FiLe://LoCaLhOsT/D:/a%23b", r"D:\a#b"),
            ("file:///C:/a%23b%3Fc", r"C:\a#b?c"),
            ("file:///C:/%2520%252F%255C%2500", r"C:\%20%2F%5C%00"),
            ("file:///C:/%25", r"C:\%"),
            ("file:///C:/a+b%2Bc", r"C:\a+b+c"),
            ("file:///%63%3a/caf%C3%A9/東京", r"c:\café\東京"),
            ("file:///C:/%EE%80%80", "C:\\\u{e000}"),
            ("file:///D:/dir/./sub/../file", r"D:\dir\.\sub\..\file"),
        ] {
            for iri in representations(input) {
                for path in [iri.try_to_path().unwrap(), iri.to_path().unwrap()] {
                    assert!(path.is_absolute(), "{input}");
                    assert_eq!(path.as_os_str(), std::ffi::OsStr::new(expected), "{input}");
                }
            }
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn to_path_requires_windows_drive_roots() {
        for input in [
            "file:///",
            "file:///tmp/data",
            "file:///C:",
            "file:///C:relative",
            "file:///1:/data",
            "file:///é:/data",
            "file:///C%3Arelative",
            "file:///C%7C/data",
            "file:////server/share/data",
            "file:////./C:/data",
        ] {
            for iri in representations(input) {
                assert_eq!(
                    iri.try_to_path(),
                    Err(crate::IriToPathError::PathNotAbsolute),
                    "{input}"
                );
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn windows_unc_paths_round_trip() {
        for (input, expected, authority, path) in [
            (r"\\server\share", "file://server/share", "server", "/share"),
            (
                r"\\server\share\",
                "file://server/share/",
                "server",
                "/share/",
            ),
            (
                r"\\localhost\share\file",
                "file://localhost/share/file",
                "localhost",
                "/share/file",
            ),
            (
                r"\\server\a b\a#b%20",
                "file://server/a%20b/a%23b%2520",
                "server",
                "/a%20b/a%23b%2520",
            ),
            (
                r"\\serveur-é\共有\café",
                "file://serveur-é/共有/café",
                "serveur-é",
                "/共有/café",
            ),
            (
                r"\\host@name:80\share\file",
                "file://host%40name%3A80/share/file",
                "host%40name%3A80",
                "/share/file",
            ),
            (
                r"\\host%20\share\file",
                "file://host%2520/share/file",
                "host%2520",
                "/share/file",
            ),
            (
                r"//server/share/./dir\..\file",
                "file://server/share/./dir/../file",
                "server",
                "/share/./dir/../file",
            ),
        ] {
            let iri = Iri::try_from(std::path::Path::new(input)).unwrap();
            assert_eq!(iri.as_str(), expected, "{input}");
            assert_eq!(iri.authority_str(), Some(authority), "{input}");
            assert_eq!(iri.path(), path, "{input}");
            assert!(!iri.has_query(), "{input}");
            assert!(!iri.has_fragment(), "{input}");
            let decoded = iri.try_to_path().unwrap();
            assert!(decoded.is_absolute(), "{input}");
            assert_eq!(
                decoded.as_os_str(),
                std::ffi::OsStr::new(&input.replace('/', "\\")),
                "{input}"
            );
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn to_path_decodes_windows_unc_paths() {
        for (input, expected) in [
            ("file://server/share", r"\\server\share"),
            ("file://server/share/", r"\\server\share\"),
            ("file://server/a%20b/c%23d", r"\\server\a b\c#d"),
            ("FiLe://SeRvEr/share/%2520", r"\\SeRvEr\share\%20"),
            ("file://127.0.0.1/share/file", r"\\127.0.0.1\share\file"),
            ("file://local%68ost/C:/Temp/file", r"C:\Temp\file"),
            ("file://LoCaLhOsT/share/file", r"\\LoCaLhOsT\share\file"),
            ("file://local%68ost/share/file", r"\\localhost\share\file"),
            ("file://h%C3%B4te/共有/caf%C3%A9", r"\\hôte\共有\café"),
            ("file://host%252F/share/%255C", r"\\host%2F\share\%5C"),
            (
                "file://server/share/./dir/../file",
                r"\\server\share\.\dir\..\file",
            ),
        ] {
            for iri in representations(input) {
                for path in [iri.try_to_path().unwrap(), iri.to_path().unwrap()] {
                    assert!(path.is_absolute(), "{input}");
                    assert_eq!(path.as_os_str(), std::ffi::OsStr::new(expected), "{input}");
                }
            }
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn to_path_rejects_invalid_windows_unc_paths() {
        use crate::IriToPathError as Error;

        for (input, expected) in [
            ("file://user@server/share", Error::UnsupportedAuthority),
            ("file://@localhost/share", Error::UnsupportedAuthority),
            ("file://server:80/share", Error::UnsupportedAuthority),
            ("file://server:/share", Error::UnsupportedAuthority),
            ("file://:80/share", Error::UnsupportedAuthority),
            ("file://[::1]/share", Error::UnsupportedAuthority),
            ("file://./share", Error::UnsupportedAuthority),
            ("file://../share", Error::UnsupportedAuthority),
            ("file://%2E/share", Error::UnsupportedAuthority),
            ("file://%3F/C:/file", Error::UnsupportedAuthority),
            ("file://host%2Fother/share", Error::EncodedSeparator),
            ("file://host%5Cother/share", Error::EncodedSeparator),
            ("file://host%00/share", Error::NulByte),
            ("file://host%FF/share", Error::InvalidEncoding),
            ("file://server", Error::PathNotAbsolute),
            ("file://server/", Error::InvalidUncShare),
            ("file://server//share", Error::InvalidUncShare),
            ("file://server/./file", Error::InvalidUncShare),
            ("file://server/%2e%2e/file", Error::InvalidUncShare),
            ("file://server/C:/file", Error::InvalidUncShare),
            ("file://localhost/C:relative", Error::InvalidUncShare),
            ("file://server/sh%2Fare/file", Error::EncodedSeparator),
            ("file://server/share/a%5Cb", Error::EncodedSeparator),
            ("file://server/share/a%00b", Error::NulByte),
            ("file://server/share/%FF", Error::InvalidEncoding),
        ] {
            for iri in representations(input) {
                assert_eq!(iri.try_to_path(), Err(expected), "{input}");
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn from_path_rejects_windows_special_prefixes() {
        for input in [
            r"\\?\C:\Temp\file",
            r"\\?\UNC\server\share\file",
            r"\\?\Volume{example}\file",
            r"\\.\PhysicalDrive0",
        ] {
            let path = std::path::Path::new(input);
            assert!(
                matches!(
                    Iri::try_from(path),
                    Err(crate::IriError::PathPrefixUnsupported(original))
                        if original.as_os_str() == path.as_os_str()
                ),
                "{input}"
            );
        }
    }

    #[cfg(all(feature = "std", windows))]
    #[test]
    fn from_path_rejects_non_unicode_windows_paths() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::Path};

        let input = OsString::from_wide(&[0x43, 0x3a, 0x5c, 0xd800]);
        let path = Path::new(&input);
        assert!(matches!(
            Iri::try_from(path),
            Err(crate::IriError::PathNotUnicode(Some(original)))
                if original.as_path() == path
        ));
    }

    #[cfg(all(feature = "std", not(windows)))]
    #[test]
    fn posix_paths_round_trip() {
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
            assert_eq!(
                iri.to_path().unwrap().as_os_str(),
                std::ffi::OsStr::new(input),
                "{input:?}"
            );
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

    #[cfg(all(feature = "std", not(windows)))]
    #[test]
    fn to_path_accepts_local_authorities() {
        for (input, expected) in [
            ("file:/tmp/data", "/tmp/data"),
            ("file:///tmp/data", "/tmp/data"),
            ("file://localhost/tmp/a%20b", "/tmp/a b"),
            ("FiLe://LoCaLhOsT/C:/Temp/a%20b", "/C:/Temp/a b"),
        ] {
            for iri in representations(input) {
                assert_eq!(
                    iri.to_path().unwrap().as_os_str(),
                    std::ffi::OsStr::new(expected),
                    "{input}"
                );
            }
        }
    }

    #[cfg(all(feature = "std", not(windows)))]
    #[test]
    fn to_path_rejects_unsupported_authorities() {
        for input in [
            "file://remote.example/share/data",
            "file://server/share/a%20b",
            "file://localhost.example/tmp/data",
            "file://localhost./tmp/data",
            "file://127.0.0.1/tmp/data",
            "file://[::1]/C:/Temp/data",
            "file://user@localhost/tmp/data",
            "file://@localhost/C:/Temp/data",
            "file://localhost:80/tmp/data",
            "file://localhost:/C:/Temp/data",
            "file://:80/tmp/data",
            "file://local%68ost/tmp/data",
        ] {
            for iri in representations(input) {
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn to_path_rejects_empty_and_rootless_paths() {
        for input in [
            "file:",
            "file://",
            "file://localhost",
            "FiLe://LOCALHOST",
            "file:relative/path",
            "file:./data",
            "file:../data",
            "file:C:relative",
            "file:C:/Temp/data",
            "file:%43%3A/Temp/data",
            "file:%5Crooted",
            "file:%2Ftmp/data",
            "file:%252Ftmp/data",
        ] {
            for iri in representations(input) {
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn to_path_rejects_queries_and_fragments() {
        for base in [
            "file:/tmp/data",
            "file:///tmp/data",
            "FiLe://localhost/C:/Temp/a%23b",
            "file://server/share/a%20b",
        ] {
            for suffix in ["?query", "#fragment", "?query#fragment", "?", "#", "?#"] {
                let input = alloc::format!("{base}{suffix}");
                for iri in representations(&input) {
                    assert!(iri.to_path().is_none(), "{input}");
                }
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn try_to_path_reports_conversion_errors() {
        use crate::IriToPathError as Error;

        for (input, expected) in [
            ("https://example.com/data", Error::UnsupportedScheme),
            (
                "file://user@remote.example/share/data",
                Error::UnsupportedAuthority,
            ),
            ("file:///tmp/data?query", Error::UnsupportedQuery),
            ("file:///tmp/data?", Error::UnsupportedQuery),
            ("file:///C:/Temp/data#fragment", Error::UnsupportedFragment),
            ("file:///C:/Temp/data#", Error::UnsupportedFragment),
            ("file:", Error::PathNotAbsolute),
            ("file:relative/path", Error::PathNotAbsolute),
            ("file:C:/Temp/data", Error::PathNotAbsolute),
            ("file:///tmp/%FF", Error::InvalidEncoding),
            ("file:///tmp/%C3", Error::InvalidEncoding),
            ("file:///tmp/a%00b", Error::NulByte),
            ("file:///C:/Temp/a%2fb", Error::EncodedSeparator),
        ] {
            for iri in representations(input) {
                assert_eq!(iri.try_to_path(), Err(expected), "{input}");
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(all(feature = "std", not(windows)))]
    #[test]
    fn to_path_decodes_path_data_once() {
        for (input, expected) in [
            ("file:///tmp/file", "/tmp/file"),
            ("file:///tmp/a%20b", "/tmp/a b"),
            ("file:///tmp/a%23b%3Fc", "/tmp/a#b?c"),
            ("file:///C:/Temp/a%23b", "/C:/Temp/a#b"),
            ("file:///tmp/a%2520b", "/tmp/a%20b"),
            ("file:///tmp/%252F%255C%2500", "/tmp/%2F%5C%00"),
            ("file:///tmp/%25", "/tmp/%"),
            ("file:///tmp/a+b%2Bc", "/tmp/a+b+c"),
            ("file:///tmp/caf%c3%a9/%E6%9D%B1%E4%BA%AC", "/tmp/café/東京"),
            ("file:///tmp/café/%EE%80%80", "/tmp/café/\u{e000}"),
            ("FiLe:/tmp/a%0ab", "/tmp/a\nb"),
        ] {
            for iri in representations(input) {
                for path in [iri.to_path().unwrap(), iri.try_to_path().unwrap()] {
                    assert_eq!(path.as_os_str(), std::ffi::OsStr::new(expected), "{input}");
                }
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn to_path_rejects_unrepresentable_path_data() {
        for input in [
            "file:///tmp/%FF",
            "file:///tmp/%C3",
            "file:///tmp/%C0%AF",
            "file:///tmp/%ED%A0%80",
            "file:///tmp/%F4%90%80%80",
            "file:///tmp/a%00b",
            "file:///tmp/a%2Fb",
            "file:///C:/Temp/a%2fb",
        ] {
            for iri in representations(input) {
                assert!(iri.to_path().is_none(), "{input}");
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn to_path_decodes_backslashes_only_off_windows() {
        for (input, expected) in [
            ("file:///tmp/a%5Cb", r"/tmp/a\b"),
            ("file:///C:/Temp/a%5cb", r"/C:/Temp/a\b"),
        ] {
            for iri in representations(input) {
                if cfg!(windows) {
                    assert!(iri.to_path().is_none(), "{input}");
                    assert_eq!(
                        iri.try_to_path(),
                        Err(crate::IriToPathError::EncodedSeparator),
                        "{input}"
                    );
                } else {
                    assert_eq!(
                        iri.to_path().unwrap().as_os_str(),
                        std::ffi::OsStr::new(expected),
                        "{input}"
                    );
                }
            }
        }
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
