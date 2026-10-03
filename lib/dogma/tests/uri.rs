// This is free and unencumbered software released into the public domain.

#![cfg(feature = "uri")]

extern crate alloc;

use alloc::string::String;
use dogma::{Iri, Uri, UriAuthority, UriError, UriResult};

#[test]
fn public_uri_constructors_are_strict() {
    let uri: dogma::enums::Uri<'_> = Uri::try_from("https://example.com/%C3%A9").unwrap();
    let authority: dogma::structs::UriAuthority<'_> = UriAuthority::try_from(&uri).unwrap();
    assert_eq!(authority.host_str(), "example.com");
    let borrowed: UriResult<Uri<'_>> = Uri::try_from("https://example.com/café");
    assert!(matches!(borrowed, Err(UriError::Invalid(None))));
    assert!(matches!(Uri::try_from(String::from("relative")),
        Err(UriError::Invalid(Some(text))) if text == "relative"));
    assert!(Iri::try_from("https://example.com/café").is_ok());
}

#[test]
fn public_iri_conversion_borrows_ascii_and_encodes_unicode() {
    for (text, expected) in [
        (
            "HTTPS://EXAMPLE.com/a/../%c3%a9?#",
            "HTTPS://EXAMPLE.com/a/../%c3%a9?#",
        ),
        ("https://example.com/café", "https://example.com/caf%C3%A9"),
        (
            "https://é@例.example/é?q=é#é",
            "https://%C3%A9@%E4%BE%8B.example/%C3%A9?q=%C3%A9#%C3%A9",
        ),
    ] {
        for iri in [Iri::try_from(text).unwrap(), text.parse().unwrap()] {
            let uri: Uri<'_> = iri.to_uri();
            assert_eq!(uri.as_str(), expected);
            assert_eq!(iri.as_str(), text);
            if text.is_ascii() {
                assert!(matches!(uri, Uri::Borrowed(_)));
                assert_eq!(uri.as_str().as_ptr(), iri.as_str().as_ptr());
                assert_eq!(Uri::try_from(&iri).unwrap(), uri);
            } else {
                assert!(matches!(uri, Uri::Owned(_)));
                assert!(Uri::try_from(&iri).is_err());
            }
            let owned: Uri<'static> = uri.into_owned();
            drop(iri);
            assert_eq!(owned.as_str(), expected);
        }
    }
}

#[test]
fn public_uri_to_iri_bridges_preserve_storage() {
    let uri: Uri<'static> = "https://example.com/%2f".parse().unwrap();
    let pointer = uri.as_str().as_ptr();
    let borrowed = Iri::from(&uri);
    assert!(matches!(borrowed, Iri::Borrowed(_)));
    assert_eq!(borrowed.as_str().as_ptr(), pointer);
    let owned: Iri<'static> = Iri::from(uri);
    assert!(matches!(owned, Iri::Owned(_)));
    assert_eq!(owned.as_str().as_ptr(), pointer);
}

#[cfg(feature = "std")]
#[test]
fn public_file_uri_round_trip() {
    #[cfg(windows)]
    let text = r"C:\café\a b%20";
    #[cfg(not(windows))]
    let text = "/café/a b%20";
    let path = std::path::Path::new(text);
    let uri = Uri::try_from(path).unwrap();
    assert!(uri.as_str().is_ascii());
    assert!(uri.as_str().ends_with("caf%C3%A9/a%20b%2520"));
    assert_eq!(uri.try_to_path().unwrap().as_os_str(), path.as_os_str());
}

#[cfg(feature = "clap")]
#[test]
fn public_clap_parser_returns_uri() {
    use clap::builder::TypedValueParser;
    use dogma::{UriScheme, UriValueParser};
    fn assert_output<P: TypedValueParser<Value = Uri<'static>>>() {}
    assert_output::<UriValueParser>();
    let matches = clap::Command::new("test")
        .arg(clap::Arg::new("uri").value_parser(UriValueParser::new(&[UriScheme::Https])))
        .try_get_matches_from(["test", "HTTPS://example.com/%C3%A9"])
        .unwrap();
    assert_eq!(
        matches.get_one::<Uri<'static>>("uri").unwrap().scheme(),
        UriScheme::Https
    );
}

#[cfg(feature = "miette")]
#[test]
fn public_uri_errors_have_uri_diagnostics() {
    use alloc::string::ToString;
    use miette::Diagnostic;
    let error = Uri::try_from("relative").unwrap_err();
    assert_eq!(error.code().unwrap().to_string(), "dogma::uri::invalid");
}
