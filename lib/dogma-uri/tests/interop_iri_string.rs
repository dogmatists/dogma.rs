#![cfg(feature = "iri-string")]

use dogma_uri::{Iri, Uri};
use iri_string::types::{IriStr, IriString, UriStr, UriString};

macro_rules! check_round_trip {
    ($identifier:ident, $slice:ident, $string:ident, $text:expr) => {{
        let text = $text;
        let upstream = $slice::new(text).unwrap();
        let borrowed = $identifier::from(upstream);
        let view: &$slice = (&borrowed).into();
        assert_eq!(view.as_str().as_ptr(), text.as_ptr());
        assert_eq!(view, upstream);
        let copied = $string::from(&borrowed);
        assert_eq!(copied.as_str(), text);
        assert_ne!(copied.as_str().as_ptr(), text.as_ptr());
        assert_eq!($string::from(borrowed).as_str(), text);

        let pointer = copied.as_str().as_ptr();
        let owned: $identifier<'static> = copied.into();
        let view: &$slice = (&owned).into();
        assert_eq!(view.as_str().as_ptr(), pointer);
        let moved = $string::from(owned);
        assert_eq!(moved.as_str().as_ptr(), pointer);
        assert_eq!(moved.as_str(), text);
    }};
}

#[test]
fn uri_storage_and_spelling_round_trip() {
    check_round_trip!(Uri, UriStr, UriString, "HTTPS://EXAMPLE.com/a/../%2f?#");
}

#[test]
fn iri_storage_and_unicode_round_trip() {
    check_round_trip!(Iri, IriStr, IriString, "HTTPS://é.example/東京/../%2f?#");
}
