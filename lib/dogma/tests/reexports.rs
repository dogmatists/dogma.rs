// This is free and unencumbered software released into the public domain.

#[cfg(feature = "named")]
#[test]
fn naming_trait_and_serde_are_forwarded() {
    extern crate alloc;
    use alloc::borrow::Cow;

    struct Named;
    impl dogma::Named for Named {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("example")
        }
    }
    let named: &dyn dogma::traits::Named = &Named;
    let named: &dyn dogma_traits::Named = named;
    assert_eq!(named.name(), "example");
    #[cfg(feature = "serde")]
    serde_test::assert_ser_tokens(named, &[serde_test::Token::Str("example")]);
}

#[cfg(feature = "collection")]
#[test]
fn collection_trait_is_the_component_trait() {
    fn len(value: &impl dogma::traits::Collection<Item = u8>) -> usize {
        dogma_traits::Collection::len(value)
    }
    assert_eq!(len(&[1, 2, 3]), 3);
    assert_eq!(dogma::Collection::len(&[1, 2, 3]), 3);
}

#[cfg(feature = "path")]
#[test]
fn path_reexports_preserve_type_identity() {
    let path: dogma::path::AncestorPath = "../..".parse().unwrap();
    let _: &dogma_path::AncestorPath = &path;
    assert_eq!(path, dogma::AncestorPath::DEPTH_2);
    #[cfg(feature = "serde")]
    serde_test::assert_tokens(&path, &[serde_test::Token::U64(2)]);
    #[cfg(feature = "std")]
    assert_eq!(path.to_std_path_buf(), std::path::Path::new("../../"));
    #[cfg(feature = "camino")]
    assert_eq!(path.to_path_buf().as_str(), "../../");
}

#[cfg(feature = "iri")]
#[test]
fn iri_reexports_preserve_type_identity() {
    let iri: dogma::uri::Iri<'_> = dogma::Iri::try_from("https://example.com/café").unwrap();
    let _: &dogma_uri::Iri<'_> = &iri;
    let _: &dogma::enums::Iri<'_> = &iri;
    #[cfg(feature = "std")]
    assert!(matches!(
        dogma::Iri::try_from(std::path::Path::new("relative")),
        Err(dogma::IriError::PathIsRelative(_))
    ));
    #[cfg(feature = "serde")]
    serde_test::assert_ser_tokens(&iri, &[serde_test::Token::Str("https://example.com/café")]);
    #[cfg(feature = "miette")]
    {
        use miette::Diagnostic;
        let error = dogma::Iri::try_from("relative").unwrap_err();
        assert_eq!(error.code().unwrap().to_string(), "dogma::iri::invalid");
    }
}

#[cfg(feature = "uri")]
#[test]
fn uri_namespace_exposes_the_component_api() {
    let iri = dogma::uri::Iri::try_from("https://example.com/café").unwrap();
    let uri: dogma_uri::Uri<'_> = iri.to_uri();
    let _: &dogma::Uri<'_> = &uri;
    assert_eq!(uri.as_str(), "https://example.com/caf%C3%A9");
    #[cfg(feature = "clap")]
    {
        let matches = clap::Command::new("test")
            .arg(clap::Arg::new("uri").value_parser(dogma::uri::UriValueParser::new(&[])))
            .try_get_matches_from(["test", "https://example.com/"])
            .unwrap();
        assert!(matches.get_one::<dogma::Uri<'static>>("uri").is_some());
    }
}

// A placeholder has no values to exercise, but its namespace must be exported.
#[cfg(feature = "uuid")]
const _: () = {
    #[allow(unused_imports)]
    use dogma::uuid;
};
