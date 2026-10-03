// This is free and unencumbered software released into the public domain.

use crate::{Iri, IriError, Uri, UriError};
use alloc::string::String;

macro_rules! impl_fluent {
    ($identifier:ident, $error:ident) => {
        /// Validates and borrows either foreign ownership form.
        impl<'a, T> TryFrom<&'a fluent_uri::$identifier<T>> for $identifier<'a>
        where
            fluent_uri::$identifier<T>: AsRef<str>,
        {
            type Error = $error;

            fn try_from(value: &'a fluent_uri::$identifier<T>) -> Result<Self, Self::Error> {
                Self::try_from(value.as_ref())
            }
        }

        /// Validates the borrowed string, retaining its original lifetime.
        impl<'a> TryFrom<fluent_uri::$identifier<&'a str>> for $identifier<'a> {
            type Error = $error;

            fn try_from(value: fluent_uri::$identifier<&'a str>) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }

        /// Validates and transfers owned storage without normalization.
        impl TryFrom<fluent_uri::$identifier<String>> for $identifier<'static> {
            type Error = $error;

            fn try_from(value: fluent_uri::$identifier<String>) -> Result<Self, Self::Error> {
                Self::try_from(value.into_string())
            }
        }

        /// Parses a borrowed view without allocating or normalizing.
        impl<'a> TryFrom<&'a $identifier<'_>> for fluent_uri::$identifier<&'a str> {
            type Error = fluent_uri::ParseError;

            fn try_from(value: &'a $identifier<'_>) -> Result<Self, Self::Error> {
                Self::parse(value.as_str())
            }
        }

        /// Parses and copies into an owned foreign identifier.
        impl TryFrom<&$identifier<'_>> for fluent_uri::$identifier<String> {
            type Error = fluent_uri::ParseError;

            fn try_from(value: &$identifier<'_>) -> Result<Self, Self::Error> {
                fluent_uri::$identifier::parse(value.as_str()).map(|value| value.to_owned())
            }
        }

        /// Moves owned storage or copies borrowed storage, then validates it.
        /// The foreign error retains the string on failure.
        impl TryFrom<$identifier<'_>> for fluent_uri::$identifier<String> {
            type Error = (fluent_uri::ParseError, String);

            fn try_from(value: $identifier<'_>) -> Result<Self, Self::Error> {
                let text = match value {
                    $identifier::Borrowed(value) => String::from(value.as_str()),
                    $identifier::Owned(value) => value.into(),
                };
                Self::parse(text)
            }
        }
    };
}

impl_fluent!(Uri, UriError);
impl_fluent!(Iri, IriError);
