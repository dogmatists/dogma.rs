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

#[cfg(feature = "std")]
impl std::net::ToSocketAddrs for IriAuthority<'_> {
    type Iter = std::vec::IntoIter<std::net::SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        use std::io::{Error, ErrorKind::InvalidInput};

        let host = self.host_str();
        let port = self
            .port()
            .or(self.scheme.to_port())
            .ok_or_else(|| Error::new(InvalidInput, "missing port"))?;

        (host, port).to_socket_addrs()
    }
}
