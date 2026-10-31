// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

extern crate alloc;

#[cfg(feature = "named")]
mod named {
    use alloc::{borrow::Cow, string::String};
    use dogma_traits::{MaybeNamed, Named};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a str);

    impl Named for Borrowed<'_> {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.0)
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a str>);

    impl MaybeNamed for MaybeBorrowed<'_> {
        fn maybe_name(&self) -> Option<Cow<'_, str>> {
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

    #[test]
    fn serialize_send_and_sync_named_objects() {
        let value = Borrowed("café");
        let tokens = [Token::Str("café")];
        assert_ser_tokens(&value as &(dyn Named + Send), &tokens);
        assert_ser_tokens(&value as &(dyn Named + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn Named + Send + Sync), &tokens);
        let value = MaybeBorrowed(None);
        let tokens = [Token::None];
        assert_ser_tokens(&value as &(dyn MaybeNamed + Send), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeNamed + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeNamed + Send + Sync), &tokens);
    }
}

#[cfg(feature = "labeled")]
mod labeled {
    use alloc::{borrow::Cow, string::String};
    use dogma_traits::{Labeled, MaybeLabeled};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a str);

    impl Labeled for Borrowed<'_> {
        fn label(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.0)
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a str>);

    impl MaybeLabeled for MaybeBorrowed<'_> {
        fn maybe_label(&self) -> Option<Cow<'_, str>> {
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

    #[test]
    fn serialize_send_and_sync_labeled_objects() {
        let value = Borrowed("東京");
        let tokens = [Token::Str("東京")];
        assert_ser_tokens(&value as &(dyn Labeled + Send), &tokens);
        assert_ser_tokens(&value as &(dyn Labeled + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn Labeled + Send + Sync), &tokens);
        let value = MaybeBorrowed(None);
        let tokens = [Token::None];
        assert_ser_tokens(&value as &(dyn MaybeLabeled + Send), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeLabeled + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeLabeled + Send + Sync), &tokens);
    }
}

#[cfg(feature = "countable")]
mod countable {
    use dogma_traits::{Countable, MaybeCountable};
    use serde_test::{assert_ser_tokens, Token};

    struct Borrowed<'a>(&'a [u8]);

    impl Countable for Borrowed<'_> {
        fn count(&self) -> usize {
            self.0.len()
        }
    }

    struct MaybeBorrowed<'a>(Option<&'a [u8]>);

    impl MaybeCountable for MaybeBorrowed<'_> {
        fn maybe_count(&self) -> Option<usize> {
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

    #[test]
    fn serialize_send_and_sync_countable_objects() {
        let value = Borrowed(&[1, 2]);
        let tokens = [Token::U64(2)];
        assert_ser_tokens(&value as &(dyn Countable + Send), &tokens);
        assert_ser_tokens(&value as &(dyn Countable + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn Countable + Send + Sync), &tokens);
        let value = MaybeBorrowed(None);
        let tokens = [Token::None];
        assert_ser_tokens(&value as &(dyn MaybeCountable + Send), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeCountable + Sync), &tokens);
        assert_ser_tokens(&value as &(dyn MaybeCountable + Send + Sync), &tokens);
    }
}
