// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use crate::{
    enums::{IriError, IriScheme, Uri},
    prelude::{fmt, str::Split, FromStr, String},
    structs::IriAuthority,
};
use iri_string::{
    components::AuthorityComponents,
    types::{IriStr, IriString},
};

#[derive(Clone, Hash, PartialEq, PartialOrd)]
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
        let iri_string = std::format!("file:{}", path);
        Ok(Self::try_from(iri_string)?)
    }
}

impl Iri<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            Iri::Borrowed(iri) => iri.as_str(),
            Iri::Owned(iri) => iri.as_str(),
        }
    }

    pub fn scheme(&self) -> IriScheme {
        self.scheme_str().parse().unwrap() // always succeeds
    }

    pub fn scheme_str(&self) -> &str {
        match self {
            Iri::Borrowed(iri) => iri.scheme_str(),
            Iri::Owned(iri) => iri.scheme_str(),
        }
    }

    pub fn has_authority(&self) -> bool {
        self.authority_str().is_some()
    }

    pub fn authority(&self) -> Option<IriAuthority> {
        IriAuthority::try_from(self).ok()
    }

    pub(crate) fn authority_components(&self) -> Option<AuthorityComponents> {
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

    pub fn to_uri(&self) -> Uri {
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
