#![cfg(feature = "url")]

use dogma_uri::{Iri, IriError, Uri, UriError};
use url::Url;

#[test]
fn conversions_preserve_url_storage() {
    let url = Url::parse("https://example.com/a%2fb?#").unwrap();
    let pointer = url.as_str().as_ptr();
    assert_eq!(Uri::try_from(&url).unwrap().as_str().as_ptr(), pointer);
    assert_eq!(Iri::try_from(&url).unwrap().as_str().as_ptr(), pointer);
    let uri: Uri<'static> = Uri::try_from(url).unwrap();
    assert_eq!(uri.as_str().as_ptr(), pointer);
    assert_eq!(Url::try_from(&uri).unwrap().as_str(), uri.as_str());
    let url = Url::try_from(uri).unwrap();
    let pointer = url.as_str().as_ptr();
    let iri: Iri<'static> = Iri::try_from(url).unwrap();
    assert_eq!(iri.as_str().as_ptr(), pointer);
    assert_eq!(
        Url::try_from(iri).unwrap().as_str(),
        "https://example.com/a%2fb?#"
    );
}

#[test]
fn whatwg_normalization_and_idna_are_explicit() {
    let uri = Uri::try_from("HTTPS://EXAMPLE.COM:443/a/../b").unwrap();
    assert_eq!(
        Url::try_from(&uri).unwrap().as_str(),
        "https://example.com/b"
    );
    assert_eq!(uri.as_str(), "HTTPS://EXAMPLE.COM:443/a/../b");
    let iri = Iri::try_from("https://é.example/café").unwrap();
    assert_eq!(
        Url::try_from(&iri).unwrap().as_str(),
        "https://xn--9ca.example/caf%C3%A9"
    );
}

#[test]
fn incompatible_syntax_returns_errors_in_both_directions() {
    for text in ["https://example.com/%GG", "x:a b"] {
        let url = Url::parse(text).unwrap();
        assert!(matches!(Uri::try_from(&url), Err(UriError::Invalid(None))));
        assert!(matches!(Iri::try_from(&url), Err(IriError::Invalid(None))));
        assert!(matches!(Uri::try_from(url.clone()), Err(UriError::Invalid(Some(s))) if s == text));
        assert!(matches!(Iri::try_from(url), Err(IriError::Invalid(Some(s))) if s == text));
    }
    let text = "https://example.com:99999/";
    assert!(Url::try_from(Uri::try_from(text).unwrap()).is_err());
    assert!(Url::try_from(Iri::try_from(text).unwrap()).is_err());
}
