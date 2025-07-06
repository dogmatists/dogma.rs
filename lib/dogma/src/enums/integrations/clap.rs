// This is free and unencumbered software released into the public domain.

extern crate alloc;

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
