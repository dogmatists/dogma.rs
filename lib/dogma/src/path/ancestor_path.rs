// This is free and unencumbered software released into the public domain.

use alloc::string::ToString;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AncestorPath(usize);

impl AncestorPath {
    /// The minimum depth of an ancestor path is 1.
    pub fn depth(&self) -> usize {
        self.0 + 1
    }
}

impl core::fmt::Display for AncestorPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", "../".repeat(self.depth()))
    }
}

#[cfg(feature = "std")]
impl From<AncestorPath> for std::path::PathBuf {
    fn from(input: AncestorPath) -> Self {
        std::path::PathBuf::from(input.to_string())
    }
}

#[cfg(feature = "camino")]
impl From<AncestorPath> for camino::Utf8PathBuf {
    fn from(input: AncestorPath) -> Self {
        camino::Utf8PathBuf::from(input.to_string())
    }
}
