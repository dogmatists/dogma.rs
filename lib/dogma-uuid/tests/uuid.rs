// This is free and unencumbered software released into the public domain.

use dogma_uuid::Uuid;

const BYTES: [u8; 16] = [
    0x67, 0xe5, 0x50, 0x44, 0x10, 0xb1, 0x42, 0x6f, 0x92, 0x47, 0xbb, 0x68, 0x0e, 0x5f, 0xe0, 0xc8,
];
const ID: Uuid = Uuid::from_bytes(BYTES);
const BORROWED: &[u8; 16] = ID.as_bytes();
const EXTRACTED: [u8; 16] = ID.into_bytes();
const TEXT: &str = "67e55044-10b1-426f-9247-bb680e5fe0c8";

#[test]
fn bytes_follow_canonical_text_order() {
    assert_eq!(BORROWED, &BYTES);
    assert_eq!(EXTRACTED, BYTES);
    assert_eq!(ID.to_string(), TEXT);
    let array: &[u8; 16] = ID.as_ref();
    let slice: &[u8] = ID.as_ref();
    assert_eq!(array, &BYTES);
    assert_eq!(slice, BYTES.as_slice());
    assert_eq!(uuid::Uuid::from(ID), uuid::Uuid::parse_str(TEXT).unwrap());
    #[cfg(feature = "alloc")]
    {
        extern crate alloc;
        let bytes: alloc::vec::Vec<u8> = ID.into();
        assert_eq!(bytes.as_slice(), BYTES.as_slice());
    }
}

#[test]
fn conversions_preserve_every_bit_without_validation() {
    // Include nil, max, and each individual bit, covering all version/variant
    // bits as well as arbitrary payloads.
    for bytes in core::iter::once([0; 16])
        .chain(core::iter::once([0xff; 16]))
        .chain((0..128).map(|bit| {
            let mut bytes = [0; 16];
            bytes[bit / 8] = 1 << (bit % 8);
            bytes
        }))
    {
        let id = Uuid::from(bytes);
        let compatible: uuid::Uuid = id.into();
        assert_eq!(compatible.as_bytes(), &bytes);
        let restored = Uuid::from(compatible);
        assert_eq!(restored, id);
        assert_eq!(<[u8; 16]>::from(restored), bytes);
    }
}

#[test]
fn default_and_ordering_use_byte_values() {
    assert_eq!(Uuid::default().into_bytes(), [0; 16]);
    let mut low = [0xff; 16];
    low[0] = 0;
    let mut high = [0; 16];
    high[0] = 1;
    assert!(Uuid::default() < Uuid::from(low));
    assert!(Uuid::from(low) < Uuid::from(high));
    assert!(Uuid::from(high) < Uuid::from([0xff; 16]));
}

#[cfg(feature = "serde")]
#[test]
fn serde_uses_untagged_text_or_bytes() {
    use serde_test::{assert_tokens, Configure, Token};

    assert_tokens(&ID.readable(), &[Token::Str(TEXT)]);
    assert_tokens(&ID.compact(), &[Token::Bytes(&BYTES)]);
}
