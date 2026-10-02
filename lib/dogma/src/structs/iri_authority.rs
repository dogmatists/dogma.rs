// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use crate::{Iri, IriScheme};
use iri_string::components::AuthorityComponents;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IriAuthority<'a> {
    scheme: IriScheme,
    components: AuthorityComponents<'a>,
}

impl<'a, 'b> TryFrom<&'a Iri<'b>> for IriAuthority<'a> {
    type Error = ();

    fn try_from(iri: &'a Iri<'b>) -> Result<Self, Self::Error> {
        iri.authority_components()
            .map(|components| IriAuthority {
                scheme: iri.scheme(),
                components,
            })
            .ok_or_else(|| ())
    }
}

impl IriAuthority<'_> {
    /// See: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1
    /// See: https://datatracker.ietf.org/doc/html/rfc7230#section-2.7.1
    pub fn userinfo(&self) -> Option<&str> {
        self.components.userinfo()
    }

    pub fn username_and_password(&self) -> Option<(&str, &str)> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => Some((userinfo, "")),
            Some(userinfo) => userinfo.split_once(':'),
        }
    }

    pub fn username(&self) -> Option<&str> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => Some(userinfo),
            Some(userinfo) => userinfo.split_once(':').map(|(username, _)| username),
        }
    }

    pub fn password(&self) -> Option<&str> {
        match self.userinfo() {
            None => None,
            Some(userinfo) if !userinfo.contains(':') => None,
            Some(userinfo) => userinfo.split_once(':').map(|(_, password)| password),
        }
    }

    pub fn host_str(&self) -> &str {
        self.components.host()
    }

    pub fn port(&self) -> Option<u16> {
        self.port_str()
            .and_then(|port_str| u16::from_str_radix(port_str, 10).ok())
    }

    pub fn port_str(&self) -> Option<&str> {
        self.components.port()
    }
}

/// Resolves the host, using the scheme's default for an absent or empty port.
///
/// Returns [`InvalidInput`](std::io::ErrorKind::InvalidInput) if a nonempty port
/// is outside `0..=65535`, or if a default is needed but unavailable.
#[cfg(feature = "std")]
impl std::net::ToSocketAddrs for IriAuthority<'_> {
    type Iter = std::vec::IntoIter<std::net::SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        use std::io::{Error, ErrorKind::InvalidInput};

        let host = self.host_str();
        let port = match self.port_str() {
            None | Some("") => self
                .scheme
                .to_port()
                .ok_or_else(|| Error::new(InvalidInput, "missing port"))?,
            Some(port) => port
                .parse::<u16>()
                .map_err(|_| Error::new(InvalidInput, "invalid port"))?,
        };

        (host, port).to_socket_addrs()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::Iri;
    use core::net::SocketAddr;
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
}
