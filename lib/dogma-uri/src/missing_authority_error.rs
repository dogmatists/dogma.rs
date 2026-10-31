// This is free and unencumbered software released into the public domain.

#[cfg(feature = "miette")]
use alloc::format;
use core::fmt;

/// An error converting an identifier without an authority component into an
/// [`IriAuthority`](crate::IriAuthority) or, with `uri`, a `UriAuthority`.
///
/// Returned by their `TryFrom` conversions; [`Iri::authority`](crate::Iri::authority)
/// and `Uri::authority` return an `Option` instead. An empty authority, as in
/// `file:///path`, is present and converts successfully. Available with `iri`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
#[cfg_attr(
    feature = "miette",
    diagnostic(
        code(dogma::authority::missing),
        help("the identifier has no `//` authority; use `authority()` to get an `Option`"),
        url(docsrs),
    )
)]
pub struct MissingAuthorityError;

impl core::error::Error for MissingAuthorityError {}

impl fmt::Display for MissingAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("identifier has no authority component")
    }
}

#[cfg(all(test, feature = "miette"))]
mod tests {
    use super::MissingAuthorityError;
    use alloc::string::ToString;
    use miette::Diagnostic;

    #[test]
    fn diagnostic_uses_authority_code() {
        let error = MissingAuthorityError;
        assert_eq!(
            error.code().unwrap().to_string(),
            "dogma::authority::missing"
        );
        assert!(error.help().is_some());
        assert!(error.url().is_some());
    }
}
