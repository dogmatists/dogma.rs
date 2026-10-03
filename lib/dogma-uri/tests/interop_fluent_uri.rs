#![cfg(feature = "fluent-uri")]

use dogma_uri::{Iri, Uri};

macro_rules! check_round_trip {
    ($identifier:ident, $text:expr) => {{
        let text = $text;
        let foreign = fluent_uri::$identifier::parse(text).unwrap();
        let borrowed = $identifier::try_from(foreign).unwrap();
        assert_eq!(borrowed.as_str().as_ptr(), text.as_ptr());
        assert_eq!($identifier::try_from(&foreign).unwrap().as_str(), text);
        let view = fluent_uri::$identifier::<&str>::try_from(&borrowed).unwrap();
        assert_eq!(view.as_str().as_ptr(), text.as_ptr());
        let copied = fluent_uri::$identifier::<String>::try_from(&borrowed).unwrap();
        assert_eq!(copied.as_str(), text);
        assert_ne!(copied.as_str().as_ptr(), text.as_ptr());
        let moved = fluent_uri::$identifier::<String>::try_from(borrowed).unwrap();
        assert_eq!(moved.as_str(), text);

        let pointer = copied.as_str().as_ptr();
        assert_eq!(
            $identifier::try_from(&copied).unwrap().as_str().as_ptr(),
            pointer
        );
        let owned: $identifier<'static> = $identifier::try_from(copied).unwrap();
        assert_eq!(owned.as_str().as_ptr(), pointer);
        let moved = fluent_uri::$identifier::<String>::try_from(owned).unwrap();
        assert_eq!(moved.as_str().as_ptr(), pointer);
        assert_eq!(moved.as_str(), text);
    }};
}

#[test]
fn matching_identifiers_preserve_storage_and_spelling() {
    for text in [
        "HTTPS://EXAMPLE.com/a/../%2f?#",
        "urn:example:item",
        "x://?#",
    ] {
        check_round_trip!(Uri, text);
        check_round_trip!(Iri, text);
    }
    check_round_trip!(Iri, "https://é.example/東京?q=é#片");
}
