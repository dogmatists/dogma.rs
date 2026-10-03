#![cfg(feature = "iref")]

use dogma_uri::{Iri, Uri};

macro_rules! check_round_trip {
    ($identifier:ident, $buffer:ident, $text:expr) => {{
        let text = $text;
        let foreign = iref::$identifier::new(text).unwrap();
        let borrowed = $identifier::try_from(foreign).unwrap();
        assert_eq!(borrowed.as_str().as_ptr(), text.as_ptr());
        let view = <&iref::$identifier>::try_from(&borrowed).unwrap();
        assert_eq!(view.as_str().as_ptr(), text.as_ptr());
        let dogma_owned = borrowed.clone().into_owned();
        let view = <&iref::$identifier>::try_from(&dogma_owned).unwrap();
        assert_eq!(view.as_str().as_ptr(), dogma_owned.as_str().as_ptr());
        let copied = iref::$buffer::try_from(&borrowed).unwrap();
        assert_eq!(copied.as_str(), text);
        assert_ne!(copied.as_str().as_ptr(), text.as_ptr());
        assert_eq!(iref::$buffer::try_from(borrowed).unwrap().as_str(), text);

        let pointer = copied.as_str().as_ptr();
        assert_eq!(
            $identifier::try_from(&copied).unwrap().as_str().as_ptr(),
            pointer
        );
        let owned: $identifier<'static> = $identifier::try_from(copied).unwrap();
        let view = <&iref::$identifier>::try_from(&owned).unwrap();
        assert_eq!(view.as_str().as_ptr(), pointer);
        let moved = iref::$buffer::try_from(owned).unwrap();
        assert_eq!(moved.as_str().as_ptr(), pointer);
        assert_eq!(moved.as_str(), text);
    }};
}

#[test]
fn matching_types_preserve_storage_and_spelling() {
    for text in [
        "HTTPS://EXAMPLE.com/a/../%2f?#",
        "urn:example:item",
        "x://?#",
        "https://example.com:99999/",
        "https://[v1.example]/",
    ] {
        check_round_trip!(Uri, UriBuf, text);
        check_round_trip!(Iri, IriBuf, text);
    }
    check_round_trip!(Iri, IriBuf, "https://é.example/東京?q=é#片");
}

#[test]
fn conversion_preserves_spelling_despite_different_equality_rules() {
    let left = Iri::try_from("https://example.com/a/../b").unwrap();
    let right = Iri::try_from("https://example.com/b").unwrap();
    assert_ne!(left, right);
    let foreign_left = <&iref::Iri>::try_from(&left).unwrap();
    let foreign_right = <&iref::Iri>::try_from(&right).unwrap();
    assert_eq!(foreign_left, foreign_right);
    assert_ne!(foreign_left.as_str(), foreign_right.as_str());
    assert_eq!(Iri::try_from(foreign_left).unwrap(), left);
    assert_eq!(Iri::try_from(foreign_right).unwrap(), right);
}
