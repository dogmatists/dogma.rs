// This is free and unencumbered software released into the public domain.

use iri_string::components::AuthorityComponents;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IriAuthority<'a>(AuthorityComponents<'a>);

impl<'a> From<AuthorityComponents<'a>> for IriAuthority<'a> {
    fn from(components: AuthorityComponents<'a>) -> Self {
        IriAuthority(components)
    }
}

impl IriAuthority<'_> {
    /// See: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1
    /// See: https://datatracker.ietf.org/doc/html/rfc7230#section-2.7.1
    pub fn userinfo(&self) -> Option<&str> {
        self.0.userinfo()
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
        self.0.host()
    }

    pub fn port(&self) -> Option<u16> {
        self.port_str()
            .and_then(|port_str| u16::from_str_radix(port_str, 10).ok())
    }

    pub fn port_str(&self) -> Option<&str> {
        self.0.port()
    }
}
