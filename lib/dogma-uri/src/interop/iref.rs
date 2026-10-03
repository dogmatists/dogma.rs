// This is free and unencumbered software released into the public domain.

use crate::{Iri, IriError, Uri, UriError};
use alloc::string::String;

macro_rules! impl_iref {
    ($identifier:ident, $buffer:ident, $error:ident, $invalid:ident) => {
        /// Validates and borrows the foreign identifier without copying.
        impl<'a> TryFrom<&'a iref::$identifier> for $identifier<'a> {
            type Error = $error;

            fn try_from(value: &'a iref::$identifier) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }

        /// Validates and borrows the foreign buffer without copying.
        impl<'a> TryFrom<&'a iref::$buffer> for $identifier<'a> {
            type Error = $error;

            fn try_from(value: &'a iref::$buffer) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }

        /// Validates and takes ownership of the foreign buffer's storage.
        impl TryFrom<iref::$buffer> for $identifier<'static> {
            type Error = $error;

            fn try_from(value: iref::$buffer) -> Result<Self, Self::Error> {
                Self::try_from(value.into_string())
            }
        }

        /// Validates a borrowed view, preserving spelling without allocating.
        impl<'a> TryFrom<&'a $identifier<'_>> for &'a iref::$identifier {
            type Error = iref::$invalid<&'a str>;

            fn try_from(value: &'a $identifier<'_>) -> Result<Self, Self::Error> {
                iref::$identifier::new(value.as_str())
            }
        }

        /// Copies and validates the string into an owned foreign buffer.
        impl TryFrom<&$identifier<'_>> for iref::$buffer {
            type Error = iref::$invalid<String>;

            fn try_from(value: &$identifier<'_>) -> Result<Self, Self::Error> {
                Self::new(String::from(value.as_str()))
            }
        }

        /// Moves owned storage or copies borrowed storage, then validates it.
        /// The foreign error retains the string on failure.
        impl TryFrom<$identifier<'_>> for iref::$buffer {
            type Error = iref::$invalid<String>;

            fn try_from(value: $identifier<'_>) -> Result<Self, Self::Error> {
                let text = match value {
                    $identifier::Borrowed(value) => String::from(value.as_str()),
                    $identifier::Owned(value) => value.into(),
                };
                Self::new(text)
            }
        }
    };
}

impl_iref!(Uri, UriBuf, UriError, InvalidUri);
impl_iref!(Iri, IriBuf, IriError, InvalidIri);
