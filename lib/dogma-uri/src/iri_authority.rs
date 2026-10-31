// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use crate::{Iri, IriScheme};
use iri_string::components::AuthorityComponents;

/// A borrowed view of an IRI's authority, with its interpreted scheme.
///
/// Available with `iri`. Construct via [`Iri::authority`] or `TryFrom<&Iri>`;
/// the latter returns `Err(())` only when the authority is absent. An empty
/// authority is valid. Component strings borrow the original IRI and preserve
/// their spelling; the stored scheme uses [`Iri::scheme`]. Equality and hashing
/// include both the scheme and the authority components.
///
/// Accessors perform no network operations. With `std`, `ToSocketAddrs` can
/// resolve the host and supply a scheme-default port.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IriAuthority<'a> {
    scheme: IriScheme,
    components: AuthorityComponents<'a>,
}

impl<'a, 'b> TryFrom<&'a Iri<'b>> for IriAuthority<'a> {
    type Error = ();

    fn try_from(iri: &'a Iri<'b>) -> Result<Self, Self::Error> {
        iri.authority_components()
            .map(|components| Self::from_components(iri.scheme(), components))
            .ok_or(())
    }
}

impl<'a> IriAuthority<'a> {
    /// Builds an authority borrowing the original identifier's components.
    pub(crate) fn from_components(scheme: IriScheme, components: AuthorityComponents<'a>) -> Self {
        Self { scheme, components }
    }

    /// Borrows user information without the trailing `@`, preserving escapes.
    ///
    /// Returns `None` if absent and `Some("")` for an empty user-info component.
    /// No percent decoding or authentication is performed.
    /// See <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1>.
    pub fn userinfo(&self) -> Option<&str> {
        self.components.userinfo()
    }

    /// Splits user information at the first literal `:`, without decoding.
    ///
    /// Returns `None` if user information is absent. If no colon is present,
    /// returns the whole user-info string and an empty password; unlike
    /// [`Self::password`], this does not distinguish a missing password from an
    /// empty one. Later colons belong to the password; `%3A` is not a delimiter.
    pub fn username_and_password(&self) -> Option<(&str, &str)> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => Some((userinfo, "")),
            Some(userinfo) => userinfo.split_once(':'),
        }
    }

    /// Borrows user information before the first literal `:`, or all of it.
    ///
    /// Returns `None` only when user information is absent. An empty username
    /// is returned as `Some("")`; percent escapes remain encoded.
    pub fn username(&self) -> Option<&str> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => Some(userinfo),
            Some(userinfo) => userinfo.split_once(':').map(|(username, _)| username),
        }
    }

    /// Borrows user information after the first literal `:`, without decoding.
    ///
    /// Returns `None` when user information or the colon is absent, and
    /// `Some("")` when the colon has no following text.
    ///
    /// ```
    /// use dogma_uri::Iri;
    ///
    /// let iri = Iri::try_from("https://alice%3Abob:secret:extra@example.com/")
    ///     .unwrap();
    /// let authority = iri.authority().unwrap();
    /// assert_eq!(authority.username(), Some("alice%3Abob"));
    /// assert_eq!(authority.password(), Some("secret:extra"));
    ///
    /// let iri = Iri::try_from("https://alice@example.com/").unwrap();
    /// let authority = iri.authority().unwrap();
    /// assert_eq!(authority.password(), None);
    /// assert_eq!(authority.username_and_password(), Some(("alice", "")));
    /// ```
    pub fn password(&self) -> Option<&str> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => None,
            Some(userinfo) => userinfo.split_once(':').map(|(_, password)| password),
        }
    }

    /// Borrows the host as written, including brackets around IP literals.
    ///
    /// May be empty. Preserves case, Unicode, and percent escapes; does not
    /// perform DNS resolution, percent decoding, or IDNA conversion.
    pub fn host_str(&self) -> &str {
        self.components.host()
    }

    /// Parses an explicit nonempty port in `0..=65535`.
    ///
    /// Returns `None` for absent, empty, or out-of-range ports. Does not supply
    /// a scheme default; use [`Self::port_str`] to distinguish those cases.
    ///
    /// ```
    /// use dogma_uri::Iri;
    ///
    /// for (text, raw, port) in [
    ///     ("https://example.com/", None, None),
    ///     ("https://example.com:/", Some(""), None),
    ///     ("https://example.com:00443/", Some("00443"), Some(443)),
    ///     ("https://example.com:65536/", Some("65536"), None),
    /// ] {
    ///     let iri = Iri::try_from(text).unwrap();
    ///     let authority = iri.authority().unwrap();
    ///     assert_eq!(authority.port_str(), raw);
    ///     assert_eq!(authority.port(), port);
    /// }
    /// ```
    pub fn port(&self) -> Option<u16> {
        self.port_str()
            .and_then(|port_str| port_str.parse::<u16>().ok())
    }

    /// Borrows the explicit port without its leading `:`, preserving zeros.
    ///
    /// Returns `None` if absent and `Some("")` if empty. IRI syntax permits
    /// digit strings outside the `u16` range; this accessor preserves them.
    pub fn port_str(&self) -> Option<&str> {
        self.components.port()
    }
}

/// Resolves the host, using the scheme's default for an absent or empty port.
///
/// Bracketed IPv6 literals are parsed directly. Other hosts are percent-decoded
/// once and passed to the system resolver; no IDNA mapping is applied, so
/// Unicode hosts are resolved as written. Convert to `url::Url` with the `url`
/// feature for IDNA processing.
///
/// Returns [`InvalidInput`](std::io::ErrorKind::InvalidInput) if the host is
/// empty or decodes to invalid UTF-8 or a NUL byte, if a nonempty port is
/// outside `0..=65535`, if a default is needed but unavailable, or if a
/// bracketed IP literal is unsupported.
#[cfg(feature = "std")]
impl std::net::ToSocketAddrs for IriAuthority<'_> {
    type Iter = std::vec::IntoIter<std::net::SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        use core::net::{Ipv6Addr, SocketAddr};
        use std::io::{Error, ErrorKind::InvalidInput};

        let host = self.host_str();
        if host.is_empty() {
            return Err(Error::new(InvalidInput, "empty host"));
        }
        let port = match self.port_str() {
            None | Some("") => self
                .scheme
                .to_port()
                .ok_or_else(|| Error::new(InvalidInput, "missing port"))?,
            Some(port) => port
                .parse::<u16>()
                .map_err(|_| Error::new(InvalidInput, "invalid port"))?,
        };

        if let Some(literal) = host
            .strip_prefix('[')
            .and_then(|host| host.strip_suffix(']'))
        {
            let address = literal
                .parse::<Ipv6Addr>()
                .map_err(|_| Error::new(InvalidInput, "unsupported IP literal"))?;
            return Ok(alloc::vec![SocketAddr::from((address, port))].into_iter());
        }

        (decode_reg_name(host)?.as_ref(), port).to_socket_addrs()
    }
}

/// Percent-decodes a registered name once, borrowing it when unescaped.
#[cfg(feature = "std")]
fn decode_reg_name(host: &str) -> std::io::Result<alloc::borrow::Cow<'_, str>> {
    use alloc::{borrow::Cow, string::String, vec::Vec};
    use std::io::{Error, ErrorKind::InvalidInput};

    if !host.contains('%') {
        return Ok(Cow::Borrowed(host));
    }
    let invalid = || Error::new(InvalidInput, "invalid percent-encoded host");
    let mut decoded = Vec::with_capacity(host.len());
    let mut bytes = host.bytes();
    while let Some(mut byte) = bytes.next() {
        if byte == b'%' {
            let mut digit = || {
                bytes
                    .next()
                    .and_then(|digit| char::from(digit).to_digit(16))
                    .ok_or_else(invalid)
            };
            let high = digit()?;
            let low = digit()?;
            byte = ((high << 4) | low) as u8;
            if byte == 0 {
                return Err(invalid());
            }
        }
        decoded.push(byte);
    }
    String::from_utf8(decoded)
        .map(Cow::Owned)
        .map_err(|_| invalid())
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::Iri;
    use core::net::{Ipv6Addr, SocketAddr};
    use std::{io::ErrorKind, net::ToSocketAddrs};

    #[test]
    fn socket_addrs_reject_out_of_range_ports() {
        for text in [
            "http://127.0.0.1:65536/",
            "http://127.0.0.1:99999/",
            "https://127.0.0.1:65536/",
            "custom://127.0.0.1:65536/",
        ] {
            let iri = Iri::try_from(text).unwrap();
            let error = iri.authority().unwrap().to_socket_addrs().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{text}");
        }
    }

    #[test]
    fn socket_addrs_preserve_explicit_ports_and_scheme_defaults() {
        for (text, port) in [
            ("http://127.0.0.1:0/", 0),
            ("http://127.0.0.1:65535/", 65535),
            ("http://127.0.0.1:443/", 443),
            ("http://127.0.0.1/", 80),
            ("http://127.0.0.1:/", 80),
            ("https://127.0.0.1/", 443),
            ("https://127.0.0.1:/", 443),
            ("telnet://127.0.0.1/", 23),
            ("tftp://127.0.0.1/", 69),
            ("custom://127.0.0.1:8080/", 8080),
        ] {
            let iri = Iri::try_from(text).unwrap();
            let mut addresses = iri.authority().unwrap().to_socket_addrs().unwrap();
            assert_eq!(
                addresses.next(),
                Some(SocketAddr::from(([127, 0, 0, 1], port))),
                "{text}",
            );
            assert_eq!(addresses.next(), None, "{text}");
        }
    }

    #[test]
    fn socket_addrs_require_a_port_without_a_default() {
        for text in ["custom://127.0.0.1/", "custom://127.0.0.1:/"] {
            let iri = Iri::try_from(text).unwrap();
            let error = iri.authority().unwrap().to_socket_addrs().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{text}");
        }
    }

    #[test]
    fn socket_addrs_resolve_ipv6_literals() {
        for (text, port) in [
            ("http://[::1]:80/", 80),
            ("http://[::1]:8080/", 8080),
            ("http://[::1]/", 80),
            ("http://[::1]:/", 80),
            ("https://[::1]/", 443),
            ("https://[::1]:/", 443),
            ("custom://[::1]:0/", 0),
            ("custom://[::1]:65535/", 65535),
        ] {
            let iri = Iri::try_from(text).unwrap();
            let authority = iri.authority().unwrap();
            let mut addresses = authority.to_socket_addrs().unwrap();
            assert_eq!(
                addresses.next(),
                Some(SocketAddr::from((Ipv6Addr::LOCALHOST, port))),
                "{text}",
            );
            assert_eq!(addresses.next(), None, "{text}");
            assert_eq!(authority.host_str(), "[::1]");
        }
    }

    #[test]
    fn socket_addrs_preserve_ipv6_literal_spelling() {
        let iri = Iri::try_from("http://[2001:DB8:0:0:0:0:0:1]:8080/").unwrap();
        let authority = iri.authority().unwrap();
        let mut addresses = authority.to_socket_addrs().unwrap();
        let address = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        assert_eq!(addresses.next(), Some(SocketAddr::from((address, 8080))));
        assert_eq!(addresses.next(), None);
        assert_eq!(authority.host_str(), "[2001:DB8:0:0:0:0:0:1]");
    }

    #[test]
    fn socket_addrs_decode_registered_names_once() {
        for text in ["http://%6Cocalhost/", "http://LOCAL%68ost:80/"] {
            let iri = Iri::try_from(text).unwrap();
            let authority = iri.authority().unwrap();
            let addresses = authority.to_socket_addrs().unwrap();
            assert!(addresses.into_iter().all(|address| address.port() == 80));
            assert!(authority.host_str().contains('%'), "{text}");
        }
        // `%25` decodes to a literal `%`, not a second escape.
        assert_eq!(
            super::decode_reg_name("%256Cocalhost").unwrap(),
            "%6Cocalhost"
        );
        assert!(matches!(
            super::decode_reg_name("localhost").unwrap(),
            alloc::borrow::Cow::Borrowed("localhost")
        ));
    }

    #[test]
    fn socket_addrs_reject_empty_and_undecodable_hosts() {
        for text in [
            "http:///",
            "http://:80/",
            "http://user@/",
            "http://local%00host/",
            "http://local%FFhost/",
        ] {
            let iri = Iri::try_from(text).unwrap();
            let error = iri.authority().unwrap().to_socket_addrs().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{text}");
        }
    }

    #[test]
    fn socket_addrs_reject_unsupported_ip_literals() {
        let iri = Iri::try_from("http://[v1.test:addr]:80/").unwrap();
        let error = iri.authority().unwrap().to_socket_addrs().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
    }
}
