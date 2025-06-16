# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.9 - 2025-06-16
### Changed
- Fix an MSRV conflict with `known-schemes`

## 0.1.8 - 2025-05-09
### Changed
- Depend on known-schemes for URI/IRI schemes

## 0.1.7 - 2025-05-07
### Added
- Export more prelude types

## 0.1.6 - 2025-05-01
### Fixed
- Build issues

## 0.1.5 - 2025-04-28
### Added
- [`IriAuthority`], [`IriError`]
- `ToSocketAddrs` support

## 0.1.4 - 2025-04-27
### Added
- [`Iri`], [`IriScheme`] enums
- Feature flags for enums

## 0.1.3 - 2025-04-26
### Added
- [`Collection`], [`CollectionMut`] traits
- Feature flags for traits

## 0.1.2 - 2025-04-26

## 0.1.1 - 2024-10-16
### Added
- Optional `serde` feature
### Changed
- Provide defaults for `Maybe` traits

## 0.1.0 - 2024-09-08
### Added
- [`Countable`], [`MaybeCountable`] traits
- [`Named`], [`MaybeNamed`] traits
- [`Labeled`], [`MaybeLabeled`] traits

[`Collection`]: https://docs.rs/dogma/latest/dogma/traits/trait.Collection.html
[`CollectionMut`]: https://docs.rs/dogma/latest/dogma/traits/trait.CollectionMut.html
[`Countable`]: https://docs.rs/dogma/latest/dogma/traits/trait.Countable.html
[`Labeled`]: https://docs.rs/dogma/latest/dogma/traits/trait.Labeled.html
[`MaybeCountable`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeCountable.html
[`MaybeLabeled`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeLabeled.html
[`MaybeNamed`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeNamed.html
[`Named`]: https://docs.rs/dogma/latest/dogma/traits/trait.Named.html

[`Iri`]: https://docs.rs/dogma/latest/dogma/enums/enum.Iri.html
[`IriScheme`]: https://docs.rs/dogma/latest/dogma/enums/enum.IriScheme.html
[`Uri`]: https://docs.rs/dogma/latest/dogma/enums/enum.Uri.html
[`UriScheme`]: https://docs.rs/dogma/latest/dogma/enums/enum.UriScheme.html
