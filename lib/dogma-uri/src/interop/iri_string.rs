// This is free and unencumbered software released into the public domain.

use crate::{Iri, Uri};
use iri_string::types::{IriStr, IriString, UriStr, UriString};

macro_rules! impl_string {
    ($identifier:ident, $slice:ident, $string:ident, $accessor:ident) => {
        /// Borrows either ownership form without allocating or normalizing.
        impl<'a> From<&'a $identifier<'_>> for &'a $slice {
            fn from(identifier: &'a $identifier<'_>) -> Self {
                identifier.$accessor()
            }
        }

        /// Copies the identifier into an owned validated string.
        impl From<&$identifier<'_>> for $string {
            fn from(identifier: &$identifier<'_>) -> Self {
                identifier.$accessor().into()
            }
        }

        /// Moves owned storage or copies borrowed storage, preserving spelling.
        impl From<$identifier<'_>> for $string {
            fn from(identifier: $identifier<'_>) -> Self {
                match identifier {
                    $identifier::Borrowed(value) => value.into(),
                    $identifier::Owned(value) => value,
                }
            }
        }
    };
}

impl_string!(Uri, UriStr, UriString, as_uri_str);
impl_string!(Iri, IriStr, IriString, as_iri_str);
