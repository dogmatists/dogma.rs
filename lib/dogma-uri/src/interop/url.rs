// This is free and unencumbered software released into the public domain.

use crate::{Iri, IriError, Uri, UriError};
use alloc::string::String;
use url::Url;

macro_rules! impl_url {
    ($identifier:ident, $error:ident) => {
        /// Validates and borrows the URL's serialized spelling without copying.
        impl<'a> TryFrom<&'a Url> for $identifier<'a> {
            type Error = $error;

            fn try_from(url: &'a Url) -> Result<Self, Self::Error> {
                Self::try_from(url.as_str())
            }
        }

        /// Validates the URL's serialization, taking ownership of its storage.
        impl TryFrom<Url> for $identifier<'static> {
            type Error = $error;

            fn try_from(url: Url) -> Result<Self, Self::Error> {
                Self::try_from(String::from(url))
            }
        }

        /// Parses using WHATWG rules, which can normalize or reject the input.
        impl TryFrom<&$identifier<'_>> for Url {
            type Error = url::ParseError;

            fn try_from(identifier: &$identifier<'_>) -> Result<Self, Self::Error> {
                Self::parse(identifier.as_str())
            }
        }

        /// Parses using WHATWG rules, which can normalize or reject the input.
        impl TryFrom<$identifier<'_>> for Url {
            type Error = url::ParseError;

            fn try_from(identifier: $identifier<'_>) -> Result<Self, Self::Error> {
                Self::try_from(&identifier)
            }
        }
    };
}

impl_url!(Uri, UriError);
impl_url!(Iri, IriError);
