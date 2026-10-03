// This is free and unencumbered software released into the public domain.

use crate::FromPathError;
use alloc::string::{String, ToString};
use core::num::NonZeroUsize;

/// A relative path consisting of one or more parent-directory components.
///
/// Stores only a nonzero [`depth`](Self::depth), not the original spelling.
/// The default depth is one. Conversion from `usize` rejects zero with
/// [`core::num::TryFromIntError`]. Display writes `../` once per level, always
/// including the trailing slash, on every platform.
///
/// # Parsing and normalization
///
/// String parsing accepts both `/` and `\` separators on every platform. It
/// ignores `.` components and repeated or trailing separators, counting each
/// `..` component. Empty or current-directory-only input yields
/// [`FromPathError::Empty`]. Roots, drive prefixes, and named components yield
/// [`FromPathError::NotAncestor`]; `child/..` is rejected, not simplified.
/// Parsing does not access the filesystem or resolve symbolic links.
///
/// Conversions from `std::path::Path` and `camino::Utf8Path` instead use native
/// path components. In particular, `..\..` is accepted as two parents on Windows
/// but rejected as a named component on POSIX systems.
///
/// Available with `alloc`. Native standard-library path conversions and
/// filesystem queries require `std`; Camino conversions require `camino`.
///
/// # Serialization
///
/// With `serde`, the wire representation is the positive integer depth, not a
/// path string or a tagged object. For example, `../../` serializes as `2`.
/// This keeps serialized size independent of the number of parent components.
/// Valid depths are `1..=usize::MAX` on the receiving platform.
/// Deserialization rejects zero, negative and out-of-range integers, and
/// non-integer representations. It does not allocate a path string.
///
/// ```
/// use dogma::{AncestorPath, FromPathError};
///
/// let path: AncestorPath = r".\..\..".parse().unwrap();
/// assert_eq!(path.depth(), 2);
/// assert_eq!(path.to_string(), "../../");
/// assert_eq!(
///     "child/..".parse::<AncestorPath>(),
///     Err(FromPathError::NotAncestor),
/// );
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AncestorPath(NonZeroUsize);

impl Default for AncestorPath {
    fn default() -> Self {
        Self::DEPTH_1
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AncestorPath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AncestorPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        NonZeroUsize::deserialize(deserializer).map(Self)
    }
}

impl AncestorPath {
    /// One parent-directory component (`../`), also the default value.
    pub const DEPTH_1: Self = Self(NonZeroUsize::new(1).unwrap());
    /// Two parent-directory components (`../../`).
    pub const DEPTH_2: Self = Self(NonZeroUsize::new(2).unwrap());
    /// Three parent-directory components.
    pub const DEPTH_3: Self = Self(NonZeroUsize::new(3).unwrap());
    /// Four parent-directory components.
    pub const DEPTH_4: Self = Self(NonZeroUsize::new(4).unwrap());
    /// Five parent-directory components.
    pub const DEPTH_5: Self = Self(NonZeroUsize::new(5).unwrap());
    /// Six parent-directory components.
    pub const DEPTH_6: Self = Self(NonZeroUsize::new(6).unwrap());
    /// Seven parent-directory components.
    pub const DEPTH_7: Self = Self(NonZeroUsize::new(7).unwrap());
    /// Eight parent-directory components.
    pub const DEPTH_8: Self = Self(NonZeroUsize::new(8).unwrap());
    /// Nine parent-directory components.
    pub const DEPTH_9: Self = Self(NonZeroUsize::new(9).unwrap());

    /// Returns the number of parent-directory components, in `1..=usize::MAX`.
    ///
    /// This is the stored lexical depth, independent of the filesystem or the
    /// number of ancestors available from a particular working directory.
    pub const fn depth(&self) -> usize {
        self.0.get()
    }

    /// Always returns `false`: an ancestor path has no root or drive prefix.
    pub const fn is_absolute(&self) -> bool {
        false
    }

    /// Always returns `true`: parent-directory components form a relative path.
    pub const fn is_relative(&self) -> bool {
        true
    }

    /// Returns `true` if this ancestor path exists and points to a directory.
    ///
    /// The path is relative to the process's current working directory.
    /// Returns `false` if metadata cannot be read.
    #[cfg(feature = "std")]
    pub fn is_dir(&self) -> bool {
        self.to_std_path_buf().is_dir()
    }

    /// Returns `true` if this ancestor path exists.
    ///
    /// The path is relative to the process's current working directory.
    /// Returns `false` if its existence cannot be determined. Use
    /// [`Self::try_exists`] to distinguish errors from a missing path.
    #[cfg(feature = "std")]
    pub fn exists(&self) -> bool {
        self.to_std_path_buf().exists()
    }

    /// Checks whether this ancestor path exists.
    ///
    /// The path is relative to the process's current working directory.
    /// Returns an error if its existence cannot be determined.
    #[cfg(feature = "std")]
    pub fn try_exists(&self) -> std::io::Result<bool> {
        self.to_std_path_buf().try_exists()
    }

    /// Allocates a [`std::path::PathBuf`] containing `../` once per depth level.
    ///
    /// Uses forward slashes and a trailing slash even on Windows. The path stays
    /// relative; this conversion does not access the filesystem. Requires `std`.
    #[cfg(feature = "std")]
    pub fn to_std_path_buf(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.to_string())
    }

    /// Allocates a [`camino::Utf8PathBuf`] containing `../` once per depth level.
    ///
    /// Uses forward slashes and a trailing slash on every platform. The path
    /// stays relative; no filesystem access is performed. Requires `camino`.
    #[cfg(feature = "camino")]
    pub fn to_path_buf(&self) -> camino::Utf8PathBuf {
        camino::Utf8PathBuf::from(self.to_string())
    }

    /// Consumes this ancestor path and allocates a [`std::path::PathBuf`].
    ///
    /// Produces the same relative path as [`Self::to_std_path_buf`]. Consuming
    /// the value does not avoid allocation: it stores only a depth. Requires
    /// `std`.
    #[cfg(feature = "std")]
    pub fn into_std_path_buf(self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.into_string())
    }

    /// Consumes this ancestor path and allocates a [`camino::Utf8PathBuf`].
    ///
    /// Produces the same relative path as [`Self::to_path_buf`]. Consuming the
    /// value does not avoid allocation: it stores only a depth. Requires `camino`.
    #[cfg(feature = "camino")]
    pub fn into_path_buf(self) -> camino::Utf8PathBuf {
        camino::Utf8PathBuf::from(self.into_string())
    }

    /// Consumes this ancestor path and allocates its canonical string.
    ///
    /// The result contains `../` once per depth level, including the trailing
    /// slash. Its length is three bytes per level, so storage grows with depth.
    /// This formats the stored depth rather than recovering the original input.
    pub fn into_string(self) -> String {
        self.to_string()
    }
}

impl From<NonZeroUsize> for AncestorPath {
    fn from(depth: NonZeroUsize) -> Self {
        AncestorPath(depth)
    }
}

impl TryFrom<usize> for AncestorPath {
    type Error = core::num::TryFromIntError;

    /// Creates an ancestor path with the given depth, returning an error for zero.
    fn try_from(depth: usize) -> Result<Self, Self::Error> {
        NonZeroUsize::try_from(depth).map(Self)
    }
}

impl core::fmt::Display for AncestorPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for _ in 0..self.depth() {
            f.write_str("../")?;
        }
        Ok(())
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
        for comp in input.split(['/', '\\']) {
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

        Ok(Self(depth.try_into().unwrap()))
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

        Ok(Self(depth.try_into().unwrap()))
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

        Ok(Self(depth.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn try_from_usize_rejects_zero() {
        assert!(AncestorPath::try_from(0usize).is_err());
    }

    #[test]
    fn try_from_usize_preserves_nonzero_depths() {
        for depth in [1usize, 2, usize::MAX / 2, usize::MAX] {
            let path = AncestorPath::try_from(depth).unwrap();
            assert_eq!(path.depth(), depth);
        }
    }

    #[test]
    fn display_uses_canonical_components() {
        for (input, expected) in [
            ("..", "../"),
            ("../..", "../../"),
            (r"..\..", "../../"),
            (r".\../..\..\", "../../../"),
        ] {
            let path = AncestorPath::from_str(input).unwrap();
            assert_eq!(path.to_string(), expected);
        }
    }

    #[test]
    fn bounded_depths_round_trip_through_posix_and_windows_strings() {
        for depth in 1usize..=256 {
            let path = AncestorPath::try_from(depth).unwrap();
            let posix = path.to_string();
            let windows = posix.replace('/', "\\");

            for input in [&posix, &windows] {
                let parsed = AncestorPath::from_str(input).unwrap();
                assert_eq!(parsed, path, "input: {input:?}");
                assert_eq!(parsed.to_string(), posix);
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn std_path_buffers_preserve_canonical_ancestor_paths() {
        for (input, expected) in [
            ("..", "../"),
            ("./..//../", "../../"),
            (r"..\..", "../../"),
            (r".\../..\..\", "../../../"),
        ] {
            let ancestor = AncestorPath::from_str(input).unwrap();
            for path in [
                ancestor.to_std_path_buf(),
                ancestor.clone().into_std_path_buf(),
                std::path::PathBuf::from(ancestor.clone()),
            ] {
                // Path equality ignores trailing separators; compare raw text.
                assert_eq!(path.as_os_str(), std::ffi::OsStr::new(expected));
                assert!(path.is_relative());
                assert_eq!(AncestorPath::try_from(path).unwrap(), ancestor);
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_std_paths_uses_native_components() {
        let windows_parents = if cfg!(windows) {
            Ok(AncestorPath::DEPTH_2)
        } else {
            Err(FromPathError::NotAncestor)
        };

        for (input, expected) in [
            ("..", Ok(AncestorPath::DEPTH_1)),
            ("../..", Ok(AncestorPath::DEPTH_2)),
            ("./..//./../", Ok(AncestorPath::DEPTH_2)),
            (r"..\..", windows_parents.clone()),
            (r"..\../", windows_parents),
            ("", Err(FromPathError::Empty)),
            (".", Err(FromPathError::Empty)),
            ("././", Err(FromPathError::Empty)),
            ("/..", Err(FromPathError::NotAncestor)),
            (r"\..", Err(FromPathError::NotAncestor)),
            (r"C:..", Err(FromPathError::NotAncestor)),
            (r"C:\..", Err(FromPathError::NotAncestor)),
            (r"\\server\share\..", Err(FromPathError::NotAncestor)),
            ("../file", Err(FromPathError::NotAncestor)),
            ("file/..", Err(FromPathError::NotAncestor)),
            ("../file/..", Err(FromPathError::NotAncestor)),
        ] {
            let path = std::path::PathBuf::from(input);
            assert_eq!(
                AncestorPath::try_from(path.as_path()),
                expected,
                "borrowed path: {input:?}"
            );
            assert_eq!(
                AncestorPath::try_from(&path),
                expected,
                "borrowed path buffer: {input:?}"
            );
            assert_eq!(
                AncestorPath::try_from(path),
                expected,
                "owned path buffer: {input:?}"
            );
        }
    }

    #[cfg(feature = "camino")]
    #[test]
    fn camino_path_buffers_preserve_canonical_ancestor_paths() {
        for (input, expected) in [
            ("..", "../"),
            ("./..//../", "../../"),
            (r"..\..", "../../"),
            (r".\../..\..\", "../../../"),
        ] {
            let ancestor = AncestorPath::from_str(input).unwrap();
            for path in [
                ancestor.to_path_buf(),
                ancestor.clone().into_path_buf(),
                camino::Utf8PathBuf::from(ancestor.clone()),
            ] {
                // Path equality ignores trailing separators; compare raw text.
                assert_eq!(path.as_str(), expected);
                assert!(path.is_relative());
                assert_eq!(AncestorPath::try_from(path).unwrap(), ancestor);
            }
        }
    }

    #[cfg(feature = "camino")]
    #[test]
    fn from_camino_paths_uses_native_components() {
        let windows_parents = if cfg!(windows) {
            Ok(AncestorPath::DEPTH_2)
        } else {
            Err(FromPathError::NotAncestor)
        };

        for (input, expected) in [
            ("..", Ok(AncestorPath::DEPTH_1)),
            ("../..", Ok(AncestorPath::DEPTH_2)),
            ("./..//./../", Ok(AncestorPath::DEPTH_2)),
            (r"..\..", windows_parents.clone()),
            (r"..\../", windows_parents),
            ("", Err(FromPathError::Empty)),
            (".", Err(FromPathError::Empty)),
            ("././", Err(FromPathError::Empty)),
            ("/..", Err(FromPathError::NotAncestor)),
            (r"\..", Err(FromPathError::NotAncestor)),
            (r"C:..", Err(FromPathError::NotAncestor)),
            (r"C:\..", Err(FromPathError::NotAncestor)),
            (r"\\server\share\..", Err(FromPathError::NotAncestor)),
            ("../café", Err(FromPathError::NotAncestor)),
            ("café/..", Err(FromPathError::NotAncestor)),
            ("../café/..", Err(FromPathError::NotAncestor)),
        ] {
            let path = camino::Utf8PathBuf::from(input);
            assert_eq!(
                AncestorPath::try_from(path.as_path()),
                expected,
                "borrowed path: {input:?}"
            );
            assert_eq!(
                AncestorPath::try_from(&path),
                expected,
                "borrowed path buffer: {input:?}"
            );
            assert_eq!(
                AncestorPath::try_from(path),
                expected,
                "owned path buffer: {input:?}"
            );
        }
    }

    #[test]
    fn display_propagates_write_errors_at_any_depth() {
        struct BoundedWriter(usize);

        impl core::fmt::Write for BoundedWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.0 = self.0.checked_sub(s.len()).ok_or(core::fmt::Error)?;
                Ok(())
            }
        }

        for depth in [3usize, usize::MAX] {
            let path = AncestorPath::try_from(depth).unwrap();
            for capacity in [0, 3] {
                let mut writer = BoundedWriter(capacity);
                assert_eq!(
                    core::fmt::write(&mut writer, format_args!("{path}")),
                    Err(core::fmt::Error)
                );
            }
        }
    }

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
