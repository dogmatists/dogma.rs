// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

extern crate alloc;

#[cfg(feature = "iri")]
mod iri {
    use alloc::string::String;
    use dogma::Iri;
    use serde::Deserialize;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_ser_tokens, Token};

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

    #[test]
    fn deserialize_iri_strings_without_normalization() {
        for text in [
            "https://example.com/",
            "https://é@例.example/café?q=東京#é",
            "HTTPS://EXAMPLE.com/a/../%c3%a9?x=%FF+#",
            "urn:example:value",
        ] {
            let expected = Iri::try_from(text).unwrap();
            for token in [
                Token::Str(text),
                Token::BorrowedStr(text),
                Token::String(text),
            ] {
                assert_de_tokens(&expected, &[token]);
            }
        }
    }

    #[test]
    fn deserialized_iri_outlives_input_and_reuses_owned_strings() {
        use serde::de::value::{Error, StrDeserializer, StringDeserializer};

        let iri: Iri<'static> = {
            let text = String::from("https://example.com/café");
            Iri::deserialize(StrDeserializer::<Error>::new(&text)).unwrap()
        };
        assert!(matches!(iri, Iri::Owned(_)));
        assert_eq!(iri.as_str(), "https://example.com/café");

        let text = String::from("https://example.com/%c3%a9");
        let pointer = text.as_ptr();
        let iri: Iri<'static> = Iri::deserialize(StringDeserializer::<Error>::new(text)).unwrap();
        assert!(matches!(iri, Iri::Owned(_)));
        assert_eq!(iri.as_str().as_ptr(), pointer);
    }

    #[test]
    fn deserialize_iri_rejects_invalid_syntax_and_non_strings() {
        for text in [
            "",
            "relative",
            "//example.com/",
            "https://example.com/%GG",
            "x:a b",
        ] {
            assert_de_tokens_error::<Iri<'static>>(
                &[Token::Str(text)],
                &alloc::format!("invalid IRI: {text}"),
            );
        }
        assert_de_tokens_error::<Iri<'static>>(
            &[Token::U64(42)],
            "invalid type: integer `42`, expected a string",
        );
        assert_de_tokens_error::<Iri<'static>>(
            &[Token::None],
            "invalid type: Option value, expected a string",
        );
    }
}
