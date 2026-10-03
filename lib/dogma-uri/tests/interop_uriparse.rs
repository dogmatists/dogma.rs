#![cfg(feature = "uriparse")]

use dogma_uri::{Iri, Uri};
use uriparse::URI;

#[test]
fn conversion_serializes_foreign_components_into_owned_identifiers() {
    let text = "https://example.com/a/../%2f?x=%FF+#";
    let (uri, iri): (Uri<'static>, Iri<'static>) = {
        let input = String::from(text);
        let foreign = URI::try_from(input.as_str()).unwrap();
        (
            Uri::try_from(&foreign).unwrap(),
            Iri::try_from(foreign).unwrap(),
        )
    };
    assert!(matches!(uri, Uri::Owned(_)));
    assert!(matches!(iri, Iri::Owned(_)));
    assert_eq!(uri.as_str(), text);
    assert_eq!(iri.as_str(), text);
    assert_eq!(URI::try_from(&uri).unwrap().to_string(), text);
    assert_eq!(URI::try_from(&iri).unwrap().to_string(), text);
    let owned_uri: URI<'static> = URI::try_from(uri).unwrap();
    let owned_iri: URI<'static> = URI::try_from(iri).unwrap();
    assert_eq!(Uri::try_from(owned_uri).unwrap().as_str(), text);
    assert_eq!(Iri::try_from(&owned_iri).unwrap().as_str(), text);
}

#[test]
fn foreign_normalization_is_visible_without_changing_the_source() {
    let text = "HTTP://example.com:00080";
    let uri = Uri::try_from(text).unwrap();
    let foreign = URI::try_from(&uri).unwrap();
    assert_eq!(foreign.to_string(), "http://example.com:80/");
    assert_eq!(uri.as_str(), text);
    assert_eq!(
        Uri::try_from(foreign).unwrap().as_str(),
        "http://example.com:80/"
    );
}

#[test]
fn incompatible_ports_and_unicode_are_rejected() {
    let text = "https://example.com:99999/";
    assert!(URI::try_from(Uri::try_from(text).unwrap()).is_err());
    assert!(URI::try_from(Iri::try_from(text).unwrap()).is_err());
    let iri = Iri::try_from("https://example.com/café").unwrap();
    assert!(URI::try_from(&iri).is_err());
    assert!(URI::try_from(iri.clone()).is_err());
    assert_eq!(
        URI::try_from(iri.to_uri()).unwrap().to_string(),
        "https://example.com/caf%C3%A9"
    );
}
