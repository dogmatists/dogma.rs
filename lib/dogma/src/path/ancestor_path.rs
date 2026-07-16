// This is free and unencumbered software released into the public domain.

use crate::FromPathError;
use alloc::string::{String, ToString};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AncestorPath(usize);

impl AncestorPath {
    /// The minimum depth of an ancestor path is 1.
    pub fn depth(&self) -> usize {
        self.0 + 1
    }

    #[cfg(feature = "std")]
    pub fn to_std_path_buf(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.to_string())
    }

    #[cfg(feature = "camino")]
    pub fn to_path_buf(&self) -> camino::Utf8PathBuf {
        camino::Utf8PathBuf::from(self.to_string())
    }

    #[cfg(feature = "std")]
    pub fn into_std_path_buf(self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.into_string())
    }

    #[cfg(feature = "camino")]
    pub fn into_path_buf(self) -> camino::Utf8PathBuf {
        camino::Utf8PathBuf::from(self.into_string())
    }

    pub fn into_string(self) -> String {
        self.to_string()
    }
}

impl core::fmt::Display for AncestorPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", "../".repeat(self.depth()))
    }
}

impl core::str::FromStr for AncestorPath {
    type Err = FromPathError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(FromPathError::Empty);
        }

        // Reject absolute POSIX or Windows-rooted paths (leading '/' or '\\')
        if input.starts_with('/') || input.starts_with('\\') {
            return Err(FromPathError::NotAncestor);
        }

        // Reject Windows drive prefixes like "C:" or "C:\" and UNC paths "\\server\share"
        if input.len() >= 2 && input.as_bytes()[1] == b':' {
            return Err(FromPathError::NotAncestor);
        }
        if input.starts_with("//") || input.starts_with("\\\\") {
            return Err(FromPathError::NotAncestor);
        }

        let mut depth: usize = 0;
        // Split on both '/' and '\\' to support POSIX and Windows separators
        for comp in input.split(|c| c == '/' || c == '\\') {
            if comp.is_empty() {
                // ignore duplicate or trailing slashes
                continue;
            }
            match comp {
                "." => continue,
                ".." => depth += 1,
                _ => return Err(FromPathError::NotAncestor),
            }
        }

        if depth == 0 {
            return Err(FromPathError::Empty);
        }

        Ok(Self(depth - 1))
    }
}

impl<T> From<&T> for AncestorPath
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

#[cfg(feature = "std")]
impl From<AncestorPath> for std::path::PathBuf {
    fn from(input: AncestorPath) -> Self {
        std::path::PathBuf::from(input.to_string())
    }
}

#[cfg(feature = "std")]
impl TryFrom<std::path::PathBuf> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<std::path::Path>::as_ref(&input))
    }
}

#[cfg(feature = "std")]
impl TryFrom<&std::path::PathBuf> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: &std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<std::path::Path>::as_ref(input))
    }
}

#[cfg(feature = "std")]
impl TryFrom<&std::path::Path> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: &std::path::Path) -> Result<Self, Self::Error> {
        use std::path::Component::*;
        let mut depth = 0;
        for component in input.components() {
            match component {
                CurDir => continue, // skip any initial "./"
                ParentDir => depth += 1,
                _ => {
                    return Err(FromPathError::NotAncestor);
                }
            }
        }
        if depth == 0 {
            return Err(FromPathError::Empty);
        }
        Ok(Self(depth - 1))
    }
}

#[cfg(feature = "camino")]
impl From<AncestorPath> for camino::Utf8PathBuf {
    fn from(input: AncestorPath) -> Self {
        camino::Utf8PathBuf::from(input.to_string())
    }
}

#[cfg(feature = "camino")]
impl TryFrom<camino::Utf8PathBuf> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: camino::Utf8PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<camino::Utf8Path>::as_ref(&input))
    }
}

#[cfg(feature = "camino")]
impl TryFrom<&camino::Utf8PathBuf> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: &camino::Utf8PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<camino::Utf8Path>::as_ref(input))
    }
}

#[cfg(feature = "camino")]
impl TryFrom<&camino::Utf8Path> for AncestorPath {
    type Error = FromPathError;

    fn try_from(input: &camino::Utf8Path) -> Result<Self, Self::Error> {
        use camino::Utf8Component::*;
        let mut depth = 0;
        for component in input.components() {
            match component {
                CurDir => continue, // skip any initial "./"
                ParentDir => depth += 1,
                _ => {
                    return Err(FromPathError::NotAncestor);
                }
            }
        }
        if depth == 0 {
            return Err(FromPathError::Empty);
        }
        Ok(Self(depth - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn from_str_posix_basic() {
        // single parent
        let p = AncestorPath::from_str("..").unwrap();
        assert_eq!(p.depth(), 1);

        // two parents
        let p = AncestorPath::from_str("../..").unwrap();
        assert_eq!(p.depth(), 2);

        // leading current dir components are ignored
        let p = AncestorPath::from_str("./..").unwrap();
        assert_eq!(p.depth(), 1);
        let p = AncestorPath::from_str("./../..").unwrap();
        assert_eq!(p.depth(), 2);

        // trailing slash is ignored
        let p = AncestorPath::from_str("../").unwrap();
        assert_eq!(p.depth(), 1);

        // embedded current dir components are ignored
        let p = AncestorPath::from_str(".././..").unwrap();
        assert_eq!(p.depth(), 2);
    }

    #[test]
    fn from_str_windows_separators() {
        // backslash separators
        let p = AncestorPath::from_str(r"..\\..").unwrap();
        assert_eq!(p.depth(), 2);

        // mixed separators are allowed
        let p = AncestorPath::from_str(r"..\\../..").unwrap();
        assert_eq!(p.depth(), 3);
    }

    #[test]
    fn from_str_errors() {
        // empty input
        match AncestorPath::from_str("") {
            Err(FromPathError::Empty) => {}
            other => panic!("expected Empty, got: {:?}", other),
        }

        // non-ancestor components
        match AncestorPath::from_str("../file") {
            Err(FromPathError::NotAncestor) => {}
            other => panic!("expected NotAncestor, got: {:?}", other),
        }

        // absolute POSIX path
        match AncestorPath::from_str("/../") {
            Err(FromPathError::NotAncestor) => {}
            other => panic!("expected NotAncestor, got: {:?}", other),
        }

        // absolute Windows UNC
        match AncestorPath::from_str(r"\\\\server\\share") {
            Err(FromPathError::NotAncestor) => {}
            other => panic!("expected NotAncestor, got: {:?}", other),
        }

        // drive-prefixed paths are rejected
        match AncestorPath::from_str(r"C:\\..\\..") {
            Err(FromPathError::NotAncestor) => {}
            other => panic!("expected NotAncestor, got: {:?}", other),
        }

        // path with file component
        match AncestorPath::from_str("../file") {
            Err(FromPathError::NotAncestor) => {}
            other => panic!("expected NotAncestor, got: {:?}", other),
        }
    }
}
