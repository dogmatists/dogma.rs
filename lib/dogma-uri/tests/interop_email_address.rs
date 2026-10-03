#![cfg(feature = "email-address")]

use dogma_uri::{Iri, MailtoError, Uri};
use email_address::EmailAddress;

#[test]
fn mailboxes_round_trip_without_losing_reserved_or_unicode_data() {
    for text in [
        "User+tag@Example.com",
        "!#$%&'*+-/=?^_`.{|}~@example.com",
        "\"a b,c@d\"@example.com",
        "user@[IPv6:2001:db8::1]",
        "用户@例子.广告",
        "percent%2F@example.com",
    ] {
        let email: EmailAddress = text.parse().unwrap();
        let uri: Uri<'static> = Uri::try_from(&email).unwrap();
        let iri: Iri<'static> = Iri::try_from(&email).unwrap();
        assert!(uri.as_str().is_ascii());
        assert_eq!(iri.as_str(), uri.as_str());
        assert!(!uri.has_authority() && !uri.has_query() && !uri.has_fragment());
        assert_eq!(EmailAddress::try_from(&uri).unwrap().as_str(), text);
        assert_eq!(EmailAddress::try_from(&iri).unwrap().as_str(), text);
        assert_eq!(EmailAddress::try_from(uri).unwrap().as_str(), text);
        assert_eq!(EmailAddress::try_from(iri).unwrap().as_str(), text);
        assert_eq!(
            Uri::try_from(email.clone()).unwrap(),
            Iri::try_from(email).unwrap().to_uri()
        );
    }
    let email: EmailAddress = "a?b#c%d@example.com".parse().unwrap();
    assert_eq!(
        Uri::try_from(email).unwrap().as_str(),
        "mailto:a%3Fb%23c%25d@example.com"
    );
}

#[test]
fn decoding_is_case_insensitive_and_single_pass() {
    for (text, expected) in [
        ("MaIlTo:user+tag@example.com", "user+tag@example.com"),
        ("mailto:user%2btag@example.com", "user+tag@example.com"),
        ("mailto:user%252F@example.com", "user%2F@example.com"),
        ("mailto:用户@例子.广告", "用户@例子.广告"),
    ] {
        let iri = Iri::try_from(text).unwrap();
        assert_eq!(EmailAddress::try_from(iri).unwrap().as_str(), expected);
    }
}

#[test]
fn unsupported_mailto_data_is_not_silently_discarded() {
    for (text, error) in [
        ("https://example.com", MailtoError::UnsupportedScheme),
        (
            "mailto://user@example.com",
            MailtoError::UnsupportedComponent,
        ),
        (
            "mailto:user@example.com?",
            MailtoError::UnsupportedComponent,
        ),
        (
            "mailto:user@example.com?subject=Hi",
            MailtoError::UnsupportedComponent,
        ),
        (
            "mailto:user@example.com#",
            MailtoError::UnsupportedComponent,
        ),
        (
            "mailto:a@example.com,b@example.com",
            MailtoError::MultipleRecipients,
        ),
        ("mailto:%FF@example.com", MailtoError::InvalidEncoding),
    ] {
        assert_eq!(
            EmailAddress::try_from(Uri::try_from(text).unwrap()),
            Err(error.clone())
        );
        assert_eq!(
            EmailAddress::try_from(Iri::try_from(text).unwrap()),
            Err(error)
        );
    }
    for text in [
        "mailto:",
        "mailto:invalid",
        "mailto:a%00b@example.com",
        "mailto:Name%20%3Ca@example.com%3E",
    ] {
        assert!(matches!(
            EmailAddress::try_from(Uri::try_from(text).unwrap()),
            Err(MailtoError::InvalidAddress(_))
        ));
    }
    for text in ["Name <a@example.com>", "invalid", "a\n@example.com"] {
        let email = EmailAddress::new_unchecked(text);
        assert!(matches!(
            Uri::try_from(&email),
            Err(MailtoError::InvalidAddress(_))
        ));
        assert!(matches!(
            Iri::try_from(email),
            Err(MailtoError::InvalidAddress(_))
        ));
    }
}
