// This is free and unencumbered software released into the public domain.

//! Public `Uuid` conversions, parsing, formatting, ordering, generation, and
//! serialization.

use dogma_uuid::{Uuid, UuidError, UuidVariant, UuidVersion};

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
fn parsing_accepts_every_upstream_form_in_either_case() {
    for text in [
        TEXT,
        "67E55044-10B1-426F-9247-BB680E5FE0C8",
        "67e5504410b1426f9247bb680e5fe0c8",
        "{67e55044-10b1-426f-9247-bb680e5fe0c8}",
        "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8",
    ] {
        assert_eq!(text.parse::<Uuid>(), Ok(ID), "{text}");
    }
}

#[test]
fn parsing_reports_invalid_text() {
    for text in [
        "",
        "67e55044",
        "67e55044-10b1-426f-9247-bb680e5fe0c",
        "67e55044-10b1-426f-9247-bb680e5fe0c8a",
        "g7e55044-10b1-426f-9247-bb680e5fe0c8",
        " 67e55044-10b1-426f-9247-bb680e5fe0c8",
    ] {
        let error: UuidError = text.parse::<Uuid>().unwrap_err();
        assert!(!error.to_string().is_empty(), "{text}");
        let _: &dyn core::error::Error = &error;
    }
}

#[test]
fn slices_must_contain_exactly_16_bytes() {
    assert_eq!(Uuid::try_from(BYTES.as_slice()), Ok(ID));
    for len in [0, 15, 17] {
        let bytes = [0xab; 17];
        assert!(Uuid::try_from(&bytes[..len]).is_err(), "{len}");
    }
}

#[test]
fn integers_follow_network_byte_order() {
    const VALUE: u128 = 0x67e5_5044_10b1_426f_9247_bb68_0e5f_e0c8;
    const FROM_VALUE: Uuid = Uuid::from_u128(VALUE);
    assert_eq!(FROM_VALUE, ID);
    assert_eq!(ID.as_u128(), VALUE);
    assert_eq!(Uuid::from(VALUE), ID);
    assert_eq!(u128::from(ID), VALUE);
    assert_eq!(Uuid::from_u128(u128::MAX), Uuid::MAX);
}

#[test]
fn nil_and_max_cover_all_bits() {
    assert_eq!(Uuid::NIL, Uuid::default());
    assert_eq!(Uuid::NIL.into_bytes(), [0; 16]);
    assert_eq!(Uuid::MAX.into_bytes(), [0xff; 16]);
    assert!(Uuid::NIL.is_nil() && !Uuid::NIL.is_max());
    assert!(Uuid::MAX.is_max() && !Uuid::MAX.is_nil());
    assert!(!ID.is_nil() && !ID.is_max());
    assert_eq!(Uuid::NIL.version(), Some(UuidVersion::Nil));
    assert_eq!(Uuid::MAX.version(), Some(UuidVersion::Max));
}

#[test]
fn version_and_variant_read_their_fields() {
    assert_eq!(ID.version(), Some(UuidVersion::Random));
    assert_eq!(ID.version_num(), 4);
    assert_eq!(ID.variant(), UuidVariant::RFC4122);
    // Version 9 is unassigned: no recognized version, but the raw field.
    let mut bytes = BYTES;
    bytes[6] = 0x90 | (bytes[6] & 0x0f);
    let unassigned = Uuid::from(bytes);
    assert_eq!(unassigned.version(), None);
    assert_eq!(unassigned.version_num(), 9);
    bytes[8] = 0xc0 | (bytes[8] & 0x1f);
    assert_eq!(Uuid::from(bytes).variant(), UuidVariant::Microsoft);
}

#[test]
fn hex_formatting_follows_case() {
    assert_eq!(format!("{ID:x}"), TEXT);
    assert_eq!(format!("{ID:X}"), TEXT.to_ascii_uppercase());
    assert_eq!(format!("{ID}"), format!("{ID:x}"));
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

#[cfg(feature = "v4")]
#[test]
fn random_generation_sets_version_and_variant() {
    let first = Uuid::new_v4();
    let second = Uuid::new_v4();
    assert_ne!(first, second);
    for id in [first, second] {
        assert_eq!(id.as_bytes()[6] >> 4, 4);
        assert_eq!(id.as_bytes()[8] >> 6, 0b10);
    }
}

#[cfg(all(feature = "v7", feature = "std"))]
#[test]
fn time_ordered_generation_uses_current_unix_milliseconds() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let before = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let first = Uuid::now_v7();
    let second = Uuid::now_v7();
    let after = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    assert!(first < second);
    for id in [first, second] {
        let bytes = id.as_bytes();
        assert_eq!(bytes[6] >> 4, 7);
        assert_eq!(bytes[8] >> 6, 0b10);
        let mut timestamp = [0; 8];
        timestamp[2..].copy_from_slice(&bytes[..6]);
        let millis = u128::from(u64::from_be_bytes(timestamp));
        assert!((before.as_millis()..=after.as_millis()).contains(&millis));
    }
}

#[cfg(feature = "serde")]
#[test]
fn serde_uses_untagged_text_or_bytes() {
    use serde_test::{Configure, Token, assert_tokens};

    assert_tokens(&ID.readable(), &[Token::Str(TEXT)]);
    assert_tokens(&ID.compact(), &[Token::Bytes(&BYTES)]);
}
