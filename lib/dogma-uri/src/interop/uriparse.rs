// This is free and unencumbered software released into the public domain.

use crate::{Iri, IriError, Uri, UriError};
use alloc::string::ToString;
use uriparse::URI;

macro_rules! impl_uriparse {
    ($identifier:ident, $error:ident) => {
        /// Serializes and validates the foreign components into owned storage.
        /// The spelling reflects uriparse's representation, not its input text.
        impl TryFrom<&URI<'_>> for $identifier<'static> {
            type Error = $error;

            fn try_from(value: &URI<'_>) -> Result<Self, Self::Error> {
                Self::try_from(value.to_string())
            }
        }

        /// Serializes and validates the foreign components into owned storage.
        impl TryFrom<URI<'_>> for $identifier<'static> {
            type Error = $error;

            fn try_from(value: URI<'_>) -> Result<Self, Self::Error> {
                Self::try_from(&value)
            }
        }

        /// Parses components borrowing the identifier where possible.
        /// Parsing may change spelling and rejects non-ASCII input.
        impl<'a> TryFrom<&'a $identifier<'_>> for URI<'a> {
            type Error = uriparse::URIError;

            fn try_from(value: &'a $identifier<'_>) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }

        /// Parses into owned foreign components, rejecting non-ASCII input.
        impl TryFrom<$identifier<'_>> for URI<'static> {
            type Error = uriparse::URIError;

            fn try_from(value: $identifier<'_>) -> Result<Self, Self::Error> {
                URI::try_from(value.as_str()).map(URI::into_owned)
            }
        }
    };
}

impl_uriparse!(Uri, UriError);
impl_uriparse!(Iri, IriError);
