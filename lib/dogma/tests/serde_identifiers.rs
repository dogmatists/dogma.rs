// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

extern crate alloc;

#[cfg(feature = "iri")]
mod iri {
    use alloc::string::String;
    use dogma::Iri;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn serialize_iri_as_a_string_independently_of_ownership() {
        for text in [
            "https://example.com/",
            "https://é@例.example/café?q=東京#é",
            "HTTPS://EXAMPLE.com/a/../%c3%a9?x=%FF+#",
            "urn:example:value",
        ] {
            let input = String::from(text);
            for iri in [
                Iri::try_from(input.as_str()).unwrap(),
                text.parse().unwrap(),
            ] {
                assert_ser_tokens(&iri, &[Token::Str(text)]);
                assert_eq!(iri.as_str(), text);
            }
        }
    }
}
