// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

use dogma_path::AncestorPath;
use serde::{de::value, Deserialize};
use serde_test::{assert_ser_tokens, assert_tokens, Token};

#[test]
fn serialize_depth_as_an_unsigned_integer() {
    for depth in [1, 2, 9, 1024, usize::MAX] {
        let path = AncestorPath::try_from(depth).unwrap();
        assert_ser_tokens(&path, &[Token::U64(depth as u64)]);
    }
}

#[test]
fn serialization_ignores_input_spelling() {
    for input in ["../..", "../../", r"..\..", "./..//./../"] {
        let path: AncestorPath = input.parse().unwrap();
        assert_ser_tokens(&path, &[Token::U64(2)]);
    }
}

#[test]
fn depths_round_trip_including_the_platform_maximum() {
    for depth in [1, 2, 9, 1024, usize::MAX] {
        let path = AncestorPath::try_from(depth).unwrap();
        assert_tokens(&path, &[Token::U64(depth as u64)]);
    }
}

#[test]
fn deserialization_preserves_the_nonzero_depth_invariant() {
    assert!(AncestorPath::deserialize(value::U64Deserializer::<value::Error>::new(0)).is_err());
    assert!(AncestorPath::deserialize(value::I64Deserializer::<value::Error>::new(-1)).is_err());
    assert!(
        AncestorPath::deserialize(value::U128Deserializer::<value::Error>::new(u128::MAX)).is_err()
    );
    #[cfg(target_pointer_width = "32")]
    assert!(
        AncestorPath::deserialize(value::U64Deserializer::<value::Error>::new(
            u64::from(u32::MAX) + 1,
        ))
        .is_err()
    );
}

#[test]
fn deserialization_rejects_noninteger_representations() {
    for text in ["2", "../../", "", "0"] {
        assert!(
            AncestorPath::deserialize(value::StrDeserializer::<value::Error>::new(text)).is_err()
        );
    }
    assert!(AncestorPath::deserialize(value::F64Deserializer::<value::Error>::new(2.0)).is_err());
    assert!(AncestorPath::deserialize(value::BoolDeserializer::<value::Error>::new(true)).is_err());
}
