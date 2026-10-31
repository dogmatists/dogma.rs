// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Iri, IriScheme};
use core::str::FromStr;

#[cfg(feature = "uri")]
pub use uri::UriValueParser;

/// Parses IRIs into owned values, optionally filtering schemes.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IriValueParser(alloc::vec::Vec<IriScheme>);

impl IriValueParser {
    /// Allows the given schemes; an empty slice allows any valid scheme.
    pub fn new(schemes: &[IriScheme]) -> Self {
        Self(schemes.to_vec())
    }
}

impl clap::builder::TypedValueParser for IriValueParser {
    type Value = Iri<'static>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        use clap::error::{Error, ErrorKind};
        let str_value = value
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;
        let iri_value = Iri::from_str(str_value).map_err(|_err| {
            clap_value_error(
                cmd,
                arg,
                str_value,
                "expected an absolute IRI, such as 'https://example.com/'",
            )
        })?;
        let schemes = self.0.iter().map(IriScheme::as_str);
        clap_check_scheme(cmd, arg, str_value, iri_value.scheme().as_str(), schemes)?;
        Ok(iri_value)
    }
}

/// Builds a value-validation error that names the argument and the rejected
/// value, with `tip` explaining the rejection.
fn clap_value_error(
    cmd: &clap::Command,
    arg: Option<&clap::Arg>,
    value: &str,
    tip: impl Into<alloc::string::String>,
) -> clap::Error {
    use alloc::string::ToString;
    use clap::error::{ContextKind, ContextValue, Error, ErrorKind};
    let arg = arg.map_or_else(|| "...".to_string(), ToString::to_string);
    let mut error = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
    error.insert(ContextKind::InvalidArg, ContextValue::String(arg));
    error.insert(
        ContextKind::InvalidValue,
        ContextValue::String(value.to_string()),
    );
    error.insert(
        ContextKind::Suggested,
        ContextValue::StyledStrs(alloc::vec![tip.into().into()]),
    );
    error
}

/// Rejects `scheme` unless `allowed` is empty or contains it, listing the
/// allowed schemes in the error.
fn clap_check_scheme<'a>(
    cmd: &clap::Command,
    arg: Option<&clap::Arg>,
    value: &str,
    scheme: &str,
    allowed: impl ExactSizeIterator<Item = &'a str> + Clone,
) -> Result<(), clap::Error> {
    if allowed.len() == 0 || allowed.clone().any(|allowed| allowed == scheme) {
        return Ok(());
    }
    let allowed = allowed.collect::<alloc::vec::Vec<_>>().join(", ");
    let tip = alloc::format!("scheme '{scheme}' is not allowed; use one of: {allowed}");
    Err(clap_value_error(cmd, arg, value, tip))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString};

    fn render(parser: IriValueParser, value: &str) -> String {
        clap::Command::new("test")
            .arg(clap::Arg::new("iri").value_parser(parser))
            .try_get_matches_from(["test", value])
            .unwrap_err()
            .render()
            .to_string()
    }

    #[test]
    fn parser_errors_name_argument_value_and_reason() {
        let message = render(IriValueParser::new(&[]), "relative/path");
        assert!(
            message.contains("invalid value 'relative/path' for '[iri]'"),
            "{message}"
        );
        assert!(
            message.contains("tip: expected an absolute IRI"),
            "{message}"
        );

        let schemes = [IriScheme::Https, IriScheme::Ftp];
        let message = render(IriValueParser::new(&schemes), "http://例.example/");
        assert!(
            message.contains("invalid value 'http://例.example/' for '[iri]'"),
            "{message}"
        );
        assert!(
            message.contains("tip: scheme 'http' is not allowed; use one of: https, ftp"),
            "{message}"
        );
    }

    #[test]
    fn parser_errors_without_argument_use_placeholder() {
        use clap::builder::TypedValueParser;
        let error = IriValueParser::new(&[])
            .parse_ref(&clap::Command::new("test"), None, "%".as_ref())
            .unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::ValueValidation);
        assert!(
            error
                .render()
                .to_string()
                .contains("invalid value '%' for '...'")
        );
    }
}

#[cfg(feature = "uri")]
mod uri {
    extern crate std;

    use crate::{Uri, UriScheme};

    /// Parses strict ASCII URIs into owned values, optionally filtering schemes.
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct UriValueParser(alloc::vec::Vec<UriScheme>);

    impl UriValueParser {
        /// Allows the given schemes; an empty slice allows any valid scheme.
        pub fn new(schemes: &[UriScheme]) -> Self {
            Self(schemes.to_vec())
        }
    }

    impl clap::builder::TypedValueParser for UriValueParser {
        type Value = Uri<'static>;

        fn parse_ref(
            &self,
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            value: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
            use clap::error::{Error, ErrorKind};

            let text = value
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;
            let uri: Uri<'static> = text.parse().map_err(|_| {
                super::clap_value_error(
                    cmd,
                    arg,
                    text,
                    "expected an absolute ASCII URI, such as 'https://example.com/'; \
                     percent-encode any Unicode",
                )
            })?;
            let schemes = self.0.iter().map(UriScheme::as_str);
            super::clap_check_scheme(cmd, arg, text, uri.scheme().as_str(), schemes)?;
            Ok(uri)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use alloc::string::{String, ToString};
        use clap::{builder::TypedValueParser, error::ErrorKind};
        use std::ffi::OsStr;

        #[test]
        fn parser_returns_owned_uri_values() {
            fn assert_output<P: TypedValueParser<Value = Uri<'static>>>() {}
            assert_output::<UriValueParser>();
            let text = "HTTPS://EXAMPLE.com/a/../%c3%a9?#";
            let uri = {
                let input = String::from(text);
                UriValueParser::new(&[])
                    .parse_ref(&clap::Command::new("test"), None, OsStr::new(&input))
                    .unwrap()
            };
            assert!(matches!(uri, Uri::Owned(_)));
            assert_eq!(uri.as_str(), text);

            let matches = clap::Command::new("test")
                .arg(clap::Arg::new("uri").value_parser(UriValueParser::new(&[])))
                .try_get_matches_from(["test", text])
                .unwrap();
            assert_eq!(matches.get_one::<Uri<'static>>("uri").unwrap(), &uri);
        }

        #[test]
        fn parser_filters_recognized_and_custom_schemes() {
            let cmd = clap::Command::new("test");
            for (schemes, text, accepted) in [
                (alloc::vec![], "x-custom:value", true),
                (alloc::vec![UriScheme::Https], "hTtPs://example.com/", true),
                (alloc::vec![UriScheme::Https], "http://example.com/", false),
                (
                    alloc::vec![UriScheme::Telnet, UriScheme::Tftp],
                    "TFTP://127.0.0.1/",
                    true,
                ),
                (
                    alloc::vec![UriScheme::Other(String::from("x-custom"))],
                    "X-Custom:value",
                    true,
                ),
                (
                    alloc::vec![UriScheme::Other(String::from("x-custom"))],
                    "x-other:value",
                    false,
                ),
            ] {
                let result = UriValueParser::new(&schemes).parse_ref(&cmd, None, OsStr::new(text));
                if accepted {
                    assert_eq!(result.unwrap().as_str(), text);
                } else {
                    assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
                }
            }
        }

        #[test]
        fn parser_rejects_unicode_and_invalid_syntax() {
            let cmd = clap::Command::new("test");
            let parser = UriValueParser::new(&[]);
            for text in [
                "https://usér@example.com/",
                "https://例.example/",
                "https://example.com/café",
                "https://example.com/?q=é",
                "https://example.com/#é",
                "/relative",
                "https://example.com/%GG",
            ] {
                assert_eq!(
                    parser
                        .parse_ref(&cmd, None, OsStr::new(text))
                        .unwrap_err()
                        .kind(),
                    ErrorKind::ValueValidation,
                    "{text}"
                );
            }
        }

        #[test]
        fn parser_errors_name_argument_value_and_reason() {
            let render = |schemes: &[UriScheme], value: &str| {
                clap::Command::new("test")
                    .arg(
                        clap::Arg::new("uri")
                            .long("uri")
                            .value_parser(UriValueParser::new(schemes)),
                    )
                    .try_get_matches_from(["test", "--uri", value])
                    .unwrap_err()
                    .render()
                    .to_string()
            };
            let message = render(&[], "https://例.example/");
            assert!(
                message.contains("invalid value 'https://例.example/' for '--uri <uri>'"),
                "{message}"
            );
            assert!(message.contains("percent-encode any Unicode"), "{message}");

            let message = render(&[UriScheme::Https], "http://example.com/");
            assert!(
                message.contains("tip: scheme 'http' is not allowed; use one of: https"),
                "{message}"
            );
        }

        #[cfg(any(unix, windows))]
        #[test]
        fn parser_reports_invalid_utf8_separately() {
            #[cfg(unix)]
            let value = {
                use std::os::unix::ffi::OsStringExt;
                std::ffi::OsString::from_vec(alloc::vec![0xff])
            };
            #[cfg(windows)]
            let value = {
                use std::os::windows::ffi::OsStringExt;
                std::ffi::OsString::from_wide(&[0xd800])
            };
            let error = UriValueParser::new(&[])
                .parse_ref(&clap::Command::new("test"), None, &value)
                .unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidUtf8);
        }
    }
}
