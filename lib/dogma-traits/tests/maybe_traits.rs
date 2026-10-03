// This is free and unencumbered software released into the public domain.

#[cfg(feature = "named")]
#[test]
fn naming_methods_can_be_used_together() {
    extern crate alloc;
    use alloc::borrow::Cow;
    use dogma_traits::{MaybeNamed, Named};

    struct Item;

    impl Named for Item {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("example")
        }
    }

    impl MaybeNamed for Item {
        fn maybe_name(&self) -> Option<Cow<'_, str>> {
            Some(self.name())
        }
    }

    fn check(item: &(impl Named + MaybeNamed)) {
        assert_eq!(item.name(), "example");
        assert_eq!(item.maybe_name().as_deref(), Some("example"));
        assert!(item.is_named());
    }

    check(&Item);
}

#[cfg(feature = "labeled")]
#[test]
fn labeling_methods_can_be_used_together() {
    extern crate alloc;
    use alloc::borrow::Cow;
    use dogma_traits::{Labeled, MaybeLabeled};

    struct Item;

    impl Labeled for Item {
        fn label(&self) -> Cow<'_, str> {
            Cow::Borrowed("example")
        }
    }

    impl MaybeLabeled for Item {
        fn maybe_label(&self) -> Option<Cow<'_, str>> {
            Some(self.label())
        }
    }

    fn check(item: &(impl Labeled + MaybeLabeled)) {
        assert_eq!(item.label(), "example");
        assert_eq!(item.maybe_label().as_deref(), Some("example"));
        assert!(item.is_labeled());
    }

    check(&Item);
}

#[cfg(feature = "countable")]
#[test]
fn counting_methods_can_be_used_together() {
    use dogma_traits::{Countable, MaybeCountable};

    struct Items(usize);

    impl Countable for Items {
        fn count(&self) -> usize {
            self.0
        }
    }

    impl MaybeCountable for Items {
        fn maybe_count(&self) -> Option<usize> {
            Some(self.count())
        }
    }

    fn check(items: &(impl Countable + MaybeCountable), count: usize) {
        assert_eq!(items.count(), count);
        assert_eq!(items.maybe_count(), Some(count));
        assert!(items.is_countable());
        assert_eq!(items.is_empty(), count == 0);
        assert_eq!(items.maybe_is_empty(), Some(count == 0));
        assert_eq!(items.is_nonempty(), count != 0);
        assert_eq!(items.maybe_is_nonempty(), Some(count != 0));
    }

    check(&Items(0), 0);
    check(&Items(3), 3);
}
