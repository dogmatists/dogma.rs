// This is free and unencumbered software released into the public domain.

use super::IriAuthority;

pub type UriAuthority<'a> = IriAuthority<'a>; // TODO

// Remove the staging module and its dead-code allowance at URI activation.
#[allow(dead_code)]
pub(crate) mod staged {
    #[cfg(feature = "std")]
    extern crate std;

    use crate::{enums::uri::staged::Uri, IriAuthority};

    /// An encoded authority borrowing its components from a validated URI.
    ///
    /// Accessors preserve spelling and percent escapes. Socket resolution
    /// shares the IRI authority's rules and requires the `std` feature.
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct UriAuthority<'a>(IriAuthority<'a>);

    impl<'a> TryFrom<&'a Uri<'_>> for UriAuthority<'a> {
        type Error = ();

        fn try_from(uri: &'a Uri<'_>) -> Result<Self, Self::Error> {
            uri.authority_components()
                .map(|components| Self(IriAuthority::from_components(uri.scheme(), components)))
                .ok_or(())
        }
    }

    impl UriAuthority<'_> {
        /// Returns encoded user information, distinguishing absent and empty.
        pub fn userinfo(&self) -> Option<&str> {
            self.0.userinfo()
        }

        /// Splits on the first literal `:`, using `""` for a missing password.
        pub fn username_and_password(&self) -> Option<(&str, &str)> {
            self.0.username_and_password()
        }

        /// Returns the encoded username, if user information is present.
        pub fn username(&self) -> Option<&str> {
            self.0.username()
        }

        /// Returns the encoded password following the first literal `:`.
        pub fn password(&self) -> Option<&str> {
            self.0.password()
        }

        /// Returns the host as written, including brackets around IP literals.
        pub fn host_str(&self) -> &str {
            self.0.host_str()
        }

        /// Parses an explicit port in `0..=65535`, without scheme defaults.
        pub fn port(&self) -> Option<u16> {
            self.0.port()
        }

        /// Returns the explicit port string, distinguishing absent and empty.
        pub fn port_str(&self) -> Option<&str> {
            self.0.port_str()
        }
    }

    /// Resolves the host with the same port and IP-literal rules as IRI authority.
    #[cfg(feature = "std")]
    impl std::net::ToSocketAddrs for UriAuthority<'_> {
        type Iter = <IriAuthority<'static> as std::net::ToSocketAddrs>::Iter;

        fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
            self.0.to_socket_addrs()
        }
    }

    #[cfg(test)]
    mod tests {
        extern crate std;

        use super::*;
        use alloc::string::String;
        use core::hash::{BuildHasher, BuildHasherDefault};
        use std::collections::hash_map::DefaultHasher;

        fn representations(text: &str) -> [Uri<'_>; 2] {
            [Uri::try_from(text).unwrap(), text.parse().unwrap()]
        }

        #[test]
        fn accessors_borrow_original_encoded_components() {
            let text = "https://u%3Aser:p%40ss:word@EXAMPLE.com:00443/path";
            for uri in representations(text) {
                let authority = uri.authority().unwrap();
                assert_eq!(authority.userinfo(), Some("u%3Aser:p%40ss:word"));
                assert_eq!(authority.username(), Some("u%3Aser"));
                assert_eq!(authority.password(), Some("p%40ss:word"));
                assert_eq!(
                    authority.username_and_password(),
                    Some(("u%3Aser", "p%40ss:word"))
                );
                assert_eq!(authority.host_str(), "EXAMPLE.com");
                assert_eq!(authority.port_str(), Some("00443"));
                assert_eq!(authority.port(), Some(443));
                let host_start = uri.as_str().find("EXAMPLE.com").unwrap();
                assert_eq!(
                    authority.host_str().as_ptr(),
                    uri.as_str()[host_start..].as_ptr()
                );
            }
        }

        #[test]
        fn authority_distinguishes_empty_and_absent_components() {
            for uri in representations("urn:example:value") {
                assert_eq!(UriAuthority::try_from(&uri), Err(()));
                assert!(uri.authority().is_none());
            }
            for (text, userinfo, pair, password, port, number) in [
                ("x://", None, None, None, None, None),
                ("x://@:", Some(""), Some(("", "")), None, Some(""), None),
                (
                    "x://user@host:0",
                    Some("user"),
                    Some(("user", "")),
                    None,
                    Some("0"),
                    Some(0),
                ),
                (
                    "x://user:@host:65535",
                    Some("user:"),
                    Some(("user", "")),
                    Some(""),
                    Some("65535"),
                    Some(65535),
                ),
                ("x://host:65536", None, None, None, Some("65536"), None),
            ] {
                for uri in representations(text) {
                    let authority = uri.authority().unwrap();
                    assert_eq!(authority.userinfo(), userinfo);
                    assert_eq!(authority.username_and_password(), pair);
                    assert_eq!(authority.username(), pair.map(|(user, _)| user));
                    assert_eq!(authority.password(), password);
                    assert_eq!(authority.port_str(), port);
                    assert_eq!(authority.port(), number);
                }
            }
        }

        #[test]
        fn authority_traits_ignore_uri_ownership() {
            let [borrowed, owned] = representations("https://example.com:443/");
            let left = borrowed.authority().unwrap();
            let right = owned.authority().unwrap();
            assert_eq!(left, right);
            assert_eq!(left.clone(), right);
            let hasher = BuildHasherDefault::<DefaultHasher>::default();
            assert_eq!(hasher.hash_one(&left), hasher.hash_one(&right));
            assert!(alloc::format!("{left:?}").starts_with("UriAuthority("));
        }

        #[test]
        fn authority_outlives_temporary_iri_view() {
            let uri = Uri::try_from(String::from("https://example.com/")).unwrap();
            let authority = {
                let iri = crate::Iri::from(&uri);
                assert_eq!(iri.authority().unwrap().host_str(), "example.com");
                uri.authority().unwrap()
            };
            assert_eq!(authority.host_str(), "example.com");
        }

        #[cfg(feature = "std")]
        #[test]
        fn socket_resolution_shares_defaults_and_ipv6_handling() {
            use core::net::{Ipv6Addr, SocketAddr};
            use std::net::ToSocketAddrs;
            for (text, expected) in [
                ("http://127.0.0.1/", SocketAddr::from(([127, 0, 0, 1], 80))),
                (
                    "https://127.0.0.1:/",
                    SocketAddr::from(([127, 0, 0, 1], 443)),
                ),
                (
                    "custom://127.0.0.1:0/",
                    SocketAddr::from(([127, 0, 0, 1], 0)),
                ),
                (
                    "https://[::1]/",
                    SocketAddr::from((Ipv6Addr::LOCALHOST, 443)),
                ),
                (
                    "custom://[::1]:65535/",
                    SocketAddr::from((Ipv6Addr::LOCALHOST, 65535)),
                ),
            ] {
                for uri in representations(text) {
                    let mut addresses = uri.authority().unwrap().to_socket_addrs().unwrap();
                    assert_eq!(addresses.next(), Some(expected));
                    assert_eq!(addresses.next(), None);
                }
            }
            for text in [
                "http://127.0.0.1:65536/",
                "custom://127.0.0.1/",
                "custom://127.0.0.1:/",
                "http://[v1.test:addr]:80/",
            ] {
                for uri in representations(text) {
                    assert_eq!(
                        uri.authority()
                            .unwrap()
                            .to_socket_addrs()
                            .unwrap_err()
                            .kind(),
                        std::io::ErrorKind::InvalidInput
                    );
                }
            }
        }
    }
}
