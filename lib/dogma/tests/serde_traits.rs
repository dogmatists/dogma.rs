// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

extern crate alloc;

#[cfg(feature = "named")]
mod named {
    use alloc::{borrow::Cow, string::String};
    use dogma::{MaybeNamed, Named};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a str);

    impl Named for Borrowed<'_> {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.0)
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a str>);

    impl MaybeNamed for MaybeBorrowed<'_> {
        fn name(&self) -> Option<Cow<'_, str>> {
            self.0.map(Cow::Borrowed)
        }
    }

    #[test]
    fn serialize_borrowed_named_objects() {
        fn check(value: &dyn Named) {
            assert_ser_tokens(value, &[Token::Str("café")]);
        }
        let text = String::from("café");
        check(&Borrowed(&text));
    }

    #[test]
    fn serialize_borrowed_maybe_named_objects() {
        fn check(value: &dyn MaybeNamed, tokens: &[Token]) {
            assert_ser_tokens(value, tokens);
        }
        let text = String::from("café");
        check(
            &MaybeBorrowed(Some(&text)),
            &[Token::Some, Token::Str("café")],
        );
        check(&MaybeBorrowed(None), &[Token::None]);
    }
}

#[cfg(feature = "labeled")]
mod labeled {
    use alloc::{borrow::Cow, string::String};
    use dogma::{Labeled, MaybeLabeled};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a str);

    impl Labeled for Borrowed<'_> {
        fn label(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.0)
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a str>);

    impl MaybeLabeled for MaybeBorrowed<'_> {
        fn label(&self) -> Option<Cow<'_, str>> {
            self.0.map(Cow::Borrowed)
        }
    }

    #[test]
    fn serialize_borrowed_labeled_objects() {
        fn check(value: &dyn Labeled) {
            assert_ser_tokens(value, &[Token::Str("東京")]);
        }
        let text = String::from("東京");
        check(&Borrowed(&text));
    }

    #[test]
    fn serialize_borrowed_maybe_labeled_objects() {
        fn check(value: &dyn MaybeLabeled, tokens: &[Token]) {
            assert_ser_tokens(value, tokens);
        }
        let text = String::from("東京");
        check(
            &MaybeBorrowed(Some(&text)),
            &[Token::Some, Token::Str("東京")],
        );
        check(&MaybeBorrowed(None), &[Token::None]);
    }
}

#[cfg(feature = "countable")]
mod countable {
    use dogma::{Countable, MaybeCountable};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a [u8]);

    impl Countable for Borrowed<'_> {
        fn count(&self) -> usize {
            self.0.len()
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a [u8]>);

    impl MaybeCountable for MaybeBorrowed<'_> {
        fn count(&self) -> Option<usize> {
            self.0.map(<[u8]>::len)
        }
    }

    #[test]
    fn serialize_borrowed_countable_objects() {
        fn check(value: &dyn Countable, count: u64) {
            assert_ser_tokens(value, &[Token::U64(count)]);
        }
        let bytes = [1, 2, 3];
        check(&Borrowed(&bytes), 3);
        check(&Borrowed(&bytes[..0]), 0);
    }

    #[test]
    fn serialize_borrowed_maybe_countable_objects() {
        fn check(value: &dyn MaybeCountable, tokens: &[Token]) {
            assert_ser_tokens(value, tokens);
        }
        let bytes = [1, 2, 3];
        check(&MaybeBorrowed(Some(&bytes)), &[Token::Some, Token::U64(3)]);
        check(
            &MaybeBorrowed(Some(&bytes[..0])),
            &[Token::Some, Token::U64(0)],
        );
        check(&MaybeBorrowed(None), &[Token::None]);
    }
}
