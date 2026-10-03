// This is free and unencumbered software released into the public domain.

#![cfg(feature = "structs")]

#[test]
fn structs_feature_exposes_both_authority_types() {
    let iri = dogma::Iri::try_from("https://例.example/path").unwrap();
    let iri_authority: dogma::structs::IriAuthority<'_> =
        dogma::IriAuthority::try_from(&iri).unwrap();
    assert_eq!(iri_authority.host_str(), "例.example");

    let uri = dogma::Uri::try_from("https://example.com/path").unwrap();
    let uri_authority: dogma::structs::UriAuthority<'_> =
        dogma::UriAuthority::try_from(&uri).unwrap();
    assert_eq!(uri_authority.host_str(), "example.com");

    for feature in ["structs", "iri", "uri", "alloc"] {
        assert!(dogma::FEATURES.contains(&feature));
    }
}
