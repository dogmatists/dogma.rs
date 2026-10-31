# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Changed
- **Breaking:** mark `IriError`, `UriError`, and `MailtoError`
  `#[non_exhaustive]`, so that enabling `std` (which adds path variants) no
  longer breaks exhaustive matches elsewhere in a build.
- **Breaking:** unify Miette diagnostic codes as `dogma::iri::*` and
  `dogma::uri::*`, replacing `dogma::iri_error::*` and `dogma::uri_error::*`.
  `IriError::PathPrefixUnsupported` and every `IriToPathError` variant
  (`dogma::iri::to_path::*`) now have codes, help text, and URLs.
- **Breaking:** `TryFrom<&Iri>` for `IriAuthority` and `TryFrom<&Uri>` for
  `UriAuthority` fail with the new `MissingAuthorityError` instead of `()`.
- **Breaking:** Upgrade `known-schemes` to 0.3. `IriScheme::Other` and
  `UriScheme::Other` now hold an `OtherScheme` (re-exported by `dogma-uri`),
  which is only constructed by parsing and is always lowercase; scheme parsing
  is infallible and case-insensitive upstream, so `Iri::scheme` and
  `Uri::scheme` delegate to it.
- **Breaking:** `dogma-uuid` conversions to and from `uuid::Uuid` now require
  its opt-in `uuid` feature, which its `arbitrary`, `atomic`, `borsh`,
  `bytemuck`, `v1`, `v3`, `v5`, `v6`, `v8`, and `zerocopy` features enable.
- `IriValueParser` and `UriValueParser` errors now name the argument and the
  rejected value, and explain the rejection, listing allowed schemes when
  filtering. The `clap` feature now enables Clap's `error-context`.
- `IriAuthority` and `UriAuthority` socket resolution now percent-decodes
  registered names once and rejects empty or undecodable hosts with
  `InvalidInput`.
- Move every crate to Rust edition 2024 and the MSRV-aware resolver 3.

### Added
- `dogma-traits`: Serialize `Send`, `Sync`, and `Send + Sync` trait objects
  of every trait that already serializes as `dyn Trait`.
- docs.rs builds every crate with all features and labels feature-gated items.
- `dogma-uuid`: Parse `Uuid` with `FromStr` and `TryFrom<&[u8]>`, failing
  with `UuidError`; add `NIL`/`MAX`, `is_nil`/`is_max`, `from_u128`/`as_u128`
  and `u128` conversions, `version`/`version_num`/`variant` (returning the
  new `UuidVersion` and `UuidVariant` enums), and `LowerHex`/`UpperHex`.
- `dogma`: The `uuid` feature includes conversions to and from `uuid::Uuid`;
  forward UUID generation with the `uuid-v4`, `uuid-v7`, and `uuid-js`
  features.
- `dogma-uri`: `Iri` and `Uri` implement `Borrow<str>`, so maps and sets keyed
  by them can be queried with string slices.
- `dogma-uri`: `Iri` and `Uri` compare with `str` and `&str` in both
  directions, by original spelling.
- `dogma-uri`: `Iri` and `Uri` compare with each other by spelling.
- `dogma-uri`: `Iri::into_string`, `Uri::into_string`, and `From` conversions
  to `String`, which move owned storage without copying.
- `dogma-path`: `AncestorPath` is `Copy` and converts into `NonZeroUsize` and
  `usize`.
- `dogma`: Document the `unstable` feature as reserved for future APIs exempt
  from semver guarantees.
- Re-export the upstream crates used in public signatures: `dogma-uri`
  re-exports `iri_string` and `known_schemes`, and `dogma-uuid` re-exports
  `uuid` with its `uuid` feature.
- Mark pure accessors, constructors, and conversions `#[must_use]`, so that
  discarding their results warns.

## 0.3.1 - 2026-10-12
### Added
- `dogma-uuid`: Generate UUIDs with `Uuid::new_v4()` and `Uuid::now_v7()`;
  enable JavaScript randomness and clock support with the `js` feature.

## 0.3.0 - 2026-10-03
### Changed
- Split implementations into `dogma-traits`, `dogma-path`, and `dogma-uri`;
  add the new `dogma-uuid`. `dogma` re-exports these crates as namespaces
  and preserves its existing type exports and feature flags.
- **Breaking:** remove the `dogma::enums` and `dogma::structs` namespaces.
  Use `dogma::uri` or the top-level type exports instead.
- **Breaking:** `Uri`, `UriError`, `UriAuthority`, and `UriValueParser` are
  distinct from their IRI counterparts. URI parsing rejects Unicode; use
  `Iri::to_uri()` to percent-encode it. This requires a breaking release.
- `Iri::to_uri()` now borrows ASCII input and encodes Unicode as UTF-8 percent
  escapes, including hostnames without IDNA conversion. URI filesystem
  construction likewise produces ASCII file URIs.
- **Breaking:** prefix optional `Maybe*` methods with `maybe_` to avoid method
  ambiguity with `Named`, `Labeled`, and `Countable`:
  - `MaybeNamed::name()` becomes `maybe_name()`.
  - `MaybeLabeled::label()` becomes `maybe_label()`.
  - `MaybeCountable::count()` becomes `maybe_count()`, `is_empty()` becomes
    `maybe_is_empty()`, and `is_nonempty()` becomes `maybe_is_nonempty()`.
### Added
- `dogma::uri`: `Uri`, `UriError`, `UriAuthority`, `UriValueParser`
- `dogma::uuid`: `Uuid`

## 0.2.2 - 2026-09-27
### Fixed
- Fix `--no-default-features` builds

## 0.2.1 - 2026-07-16
### Added
- [`dogma::path::AncestorPath`]: `#is_absolute`, `#is_relative`, `#is_dir`, `#exists`, `#try_exists`
- [`dogma::path::AncestorPath`]: `From<NonZeroUsize>`, `TryFrom<usize>`

## 0.2.0 - 2026-07-16
### Added
- [`dogma::path::AncestorPath`]
- [`dogma::path::FromPathError`]
- Integrate with [camino]

## 0.1.11 - 2025-07-06
### Added
- Integrate with [clap]

## 0.1.10 - 2025-07-06
### Changed
- Bump [known-schemes] from 0.1 to 0.2
- Bump [miette] from 7.5 to 7.6

## 0.1.9 - 2025-06-16
### Changed
- Fix an MSRV conflict with [known-schemes]

## 0.1.8 - 2025-05-09
### Changed
- Depend on [known-schemes] for URI/IRI schemes

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

[`dogma::path::AncestorPath`]: https://docs.rs/dogma/latest/dogma/path/struct.AncestorPath.html
[`dogma::path::FromPathError`]: https://docs.rs/dogma/latest/dogma/path/enum.FromPathError.html
[`Iri`]: https://docs.rs/dogma/latest/dogma/enums/enum.Iri.html
[`IriScheme`]: https://docs.rs/dogma/latest/dogma/enums/enum.IriScheme.html
[`Uri`]: https://docs.rs/dogma/latest/dogma/enums/enum.Uri.html
[`UriScheme`]: https://docs.rs/dogma/latest/dogma/enums/enum.UriScheme.html

[camino]: https://crates.io/crates/camino
[clap]: https://crates.io/crates/clap
[known-schemes]: https://crates.io/crates/known-schemes
[miette]: https://crates.io/crates/miette
