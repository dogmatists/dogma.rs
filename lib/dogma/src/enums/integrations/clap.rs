// This is free and unencumbered software released into the public domain.

extern crate alloc;

#[cfg(feature = "uri")]
pub type UriValueParser = IriValueParser;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IriValueParser(alloc::vec::Vec<IriScheme>);

impl IriValueParser {
    pub fn new(schemes: &[IriScheme]) -> Self {
        Self(schemes.to_vec())
    }
}

impl clap::builder::TypedValueParser for IriValueParser {
    type Value = Iri<'static>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        use clap::error::{Error, ErrorKind};
        let str_value = value
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;
        let iri_value = Iri::from_str(str_value)
            .map_err(|_err| Error::new(ErrorKind::ValueValidation).with_cmd(cmd))?;
        if !self.0.is_empty() && !self.0.contains(&iri_value.scheme()) {
            return Err(Error::new(ErrorKind::ValueValidation).with_cmd(cmd));
        }
        Ok(iri_value)
    }
}

// Remove the staging module and its dead-code allowance at URI activation.
#[cfg(feature = "uri")]
#[allow(dead_code)]
pub(crate) mod staged {
    extern crate std;

    use crate::{enums::uri::staged::Uri, UriScheme};

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
            _arg: Option<&clap::Arg>,
            value: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
            use clap::error::{Error, ErrorKind};

            let text = value
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;
            let uri: Uri<'static> = text
                .parse()
                .map_err(|_| Error::new(ErrorKind::ValueValidation).with_cmd(cmd))?;
            if !self.0.is_empty() && !self.0.contains(&uri.scheme()) {
                return Err(Error::new(ErrorKind::ValueValidation).with_cmd(cmd));
            }
            Ok(uri)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use alloc::string::String;
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
