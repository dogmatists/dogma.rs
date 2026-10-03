// This is free and unencumbered software released into the public domain.

use crate::{Iri, Uri};
use alloc::{string::String, vec::Vec};
use core::fmt;
use email_address::{EmailAddress, Options};

/// An error converting a single mailbox to or from a `mailto:` identifier.
/// Available with `email-address` (which requires `std`).
#[derive(Clone, Debug, PartialEq)]
pub enum MailtoError {
    /// The identifier does not use the `mailto` scheme.
    UnsupportedScheme,
    /// An authority, query, or fragment cannot be retained by a mailbox.
    UnsupportedComponent,
    /// A literal comma separates recipients; only one mailbox is supported.
    MultipleRecipients,
    /// Percent-decoded bytes are not valid UTF-8.
    InvalidEncoding,
    /// The mailbox is invalid or contains an unsupported display name.
    InvalidAddress(email_address::Error),
}

impl fmt::Display for MailtoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScheme => f.write_str("expected the mailto scheme"),
            Self::UnsupportedComponent => f.write_str("unsupported mailto component"),
            Self::MultipleRecipients => f.write_str("expected a single mailbox"),
            Self::InvalidEncoding => f.write_str("invalid UTF-8 in mailto address"),
            Self::InvalidAddress(error) => write!(f, "invalid mailto address: {error}"),
        }
    }
}

impl core::error::Error for MailtoError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::InvalidAddress(error) => Some(error),
            _ => None,
        }
    }
}

fn parse_mailbox(text: &str) -> Result<EmailAddress, MailtoError> {
    EmailAddress::parse_with_options(text, Options::default().without_display_text())
        .map_err(MailtoError::InvalidAddress)
}

fn encode_mailbox(email: &EmailAddress) -> Result<Uri<'static>, MailtoError> {
    // Revalidate: EmailAddress also exposes a safe unchecked constructor.
    parse_mailbox(email.as_str())?;
    let mut text = String::from("mailto:");
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in email.as_str().bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~@+".contains(&byte) {
            text.push(char::from(byte));
        } else {
            text.push('%');
            text.push(char::from(HEX[usize::from(byte >> 4)]));
            text.push(char::from(HEX[usize::from(byte & 15)]));
        }
    }
    // The fixed scheme and escaped ASCII path satisfy URI syntax by construction.
    Ok(Uri::try_from(text).expect("percent-encoded mailbox is a valid URI"))
}

fn decode_mailbox(iri: &Iri<'_>) -> Result<EmailAddress, MailtoError> {
    if !iri.scheme_str().eq_ignore_ascii_case("mailto") {
        return Err(MailtoError::UnsupportedScheme);
    }
    if iri.has_authority() || iri.has_query() || iri.has_fragment() {
        return Err(MailtoError::UnsupportedComponent);
    }
    if iri.path().contains(',') {
        return Err(MailtoError::MultipleRecipients);
    }
    let mut decoded = Vec::with_capacity(iri.path().len());
    let mut bytes = iri.path().bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            // IRI validation guarantees two hexadecimal digits after `%`.
            let high = char::from(bytes.next().unwrap()).to_digit(16).unwrap();
            let low = char::from(bytes.next().unwrap()).to_digit(16).unwrap();
            decoded.push(((high << 4) | low) as u8);
        } else {
            decoded.push(byte);
        }
    }
    let text = String::from_utf8(decoded).map_err(|_| MailtoError::InvalidEncoding)?;
    parse_mailbox(&text)
}

macro_rules! impl_email {
    ($identifier:ident) => {
        /// Encodes one validated mailbox as an owned ASCII `mailto:` identifier.
        /// Display names are rejected; Unicode is UTF-8 percent-encoded.
        impl TryFrom<&EmailAddress> for $identifier<'static> {
            type Error = MailtoError;

            fn try_from(email: &EmailAddress) -> Result<Self, Self::Error> {
                encode_mailbox(email).map(Into::into)
            }
        }

        /// Encodes one mailbox using the same rules as the borrowed conversion.
        impl TryFrom<EmailAddress> for $identifier<'static> {
            type Error = MailtoError;

            fn try_from(email: EmailAddress) -> Result<Self, Self::Error> {
                Self::try_from(&email)
            }
        }

        /// Decodes one mailbox, rejecting components that cannot be preserved.
        /// Percent decoding happens exactly once; `+` is never decoded as space.
        impl TryFrom<&$identifier<'_>> for EmailAddress {
            type Error = MailtoError;

            fn try_from(value: &$identifier<'_>) -> Result<Self, Self::Error> {
                decode_mailbox(&Iri::from(value.as_iri_str()))
            }
        }

        /// Decodes one mailbox using the same rules as the borrowed conversion.
        impl TryFrom<$identifier<'_>> for EmailAddress {
            type Error = MailtoError;

            fn try_from(value: $identifier<'_>) -> Result<Self, Self::Error> {
                Self::try_from(&value)
            }
        }
    };
}

impl_email!(Uri);
impl_email!(Iri);
