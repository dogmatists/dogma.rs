// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "alloc", feature = "serde"))]

use dogma::AncestorPath;
use serde_test::{assert_ser_tokens, Token};

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
