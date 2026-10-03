# Dogma.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.97%2B-blue)](https://endoflife.date/rust)
[![Package on Crates.io](https://img.shields.io/crates/v/dogma)](https://crates.io/crates/dogma)
[![Documentation](https://img.shields.io/docsrs/dogma?label=docs.rs)](https://docs.rs/dogma)

**Dogma.rs provides shared traits and identifier types for Rust.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

**Status:** pre-1.0; see the [changelog] for 0.3 breaking changes.

## ✨ Features

- Naming, labeling, counting, and collection traits.
- Validated ASCII URIs and Unicode IRIs, with borrowed or owned storage.
- Ancestor paths, file URI/IRI conversions, and socket address resolution.
- 128-bit UUIDs with lossless byte conversions and canonical formatting.
- Four independent component crates, re-exported by the `dogma` umbrella.
- Pure, safe Rust with `#![no_std]` support and `#![deny(unsafe_code)]`.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.97+

## ⬇️ Installation

```bash
cargo add dogma
```

<details>
<summary>Configuration in <code>Cargo.toml</code></summary>

All components, Serde, and `std`:

```toml
[dependencies]
dogma = "0.3"
```

Only URI/IRI support and its dependencies, without `std`:

```toml
[dependencies]
dogma = { version = "0.3", default-features = false, features = ["uri"] }
```

Or depend directly on a component:

```toml
[dependencies]
dogma-uri = "0.3"
```

</details>

## 👉 Examples

### Naming an Entity

```rust
use dogma::Named;

struct Person<'a>(&'a str);

impl Named for Person<'_> {
    fn name(&self) -> Cow<'_, str> {
        self.0.into()
    }
}

assert_eq!(Person("Ada Lovelace").name(), "Ada Lovelace");
```

## 📚 Reference

[docs.rs/dogma](https://docs.rs/dogma)

### Crates

| Crate | Namespace | Exports |
| --- | --- | --- |
| [`dogma`] | `dogma` | Umbrella for all components |
| [`dogma-traits`] | `dogma::traits` | Naming, labeling, counting, collections |
| [`dogma-path`] | `dogma::path` | `AncestorPath`, `FromPathError` |
| [`dogma-uri`] | `dogma::uri` | `Iri`, `Uri`, schemes, authorities, errors |
| [`dogma-uuid`] | `dogma::uuid` | `Uuid` |

Component exports are also available at the top level, e.g. `dogma::Iri`.

### Feature Flags

Umbrella defaults are `all` + `std`. `all` includes Serde; other interop is opt-in.

| Feature | Enables |
| --- | --- |
| `all` | All components and `serde` |
| `std` | Filesystem/network APIs, standard-library collections, `alloc` |
| `alloc` | `path`; allocation support in enabled traits/UUID components |
| `traits` | `named`, `labeled`, `countable`, `collection` |
| `named` | [`Named`], [`MaybeNamed`] |
| `labeled` | [`Labeled`], [`MaybeLabeled`] |
| `countable` | [`Countable`], [`MaybeCountable`] |
| `collection` | [`Collection`], [`CollectionMut`] |
| `path` | Ancestor paths and conversion errors |
| `iri` | Unicode IRIs, schemes, authorities; `alloc` |
| `uri` | ASCII URIs plus `iri` |
| `uuid` | UUID values and byte conversions |
| `enums`, `structs` | Compatibility aliases for `iri` + `uri` |

- For `no_std`, disable defaults and select `all` or individual features.
- Paths and identifiers require allocation; counting traits and core UUID
  operations can work without it.
- Optional trait methods use `maybe_`: `maybe_name()`, `maybe_label()`,
  `maybe_count()`, `maybe_is_empty()`, `maybe_is_nonempty()`.

### Interoperability

Features are available through `dogma` and the owning component crate.

| Feature | Version | Summary |
| --- | --- | --- |
| [`camino`] | 1.2 | `AncestorPath` conversions with UTF-8 paths |
| [`clap`] | 4.5 | `IriValueParser`, `UriValueParser`; optional scheme filters |
| [`email-address`] | 0.2.9 | Single-mailbox `mailto:` conversions with `EmailAddress` |
| [`fluent-uri`] | 0.4.1 | Checked borrowed/owned URI/IRI conversions |
| [`iref`] | 4.3 | Spelling-preserving borrowed/owned URI/IRI conversions |
| [`iri-string`] | 0.7 | Conversions to validated URI/IRI string types |
| [`miette`] | 7.6 | URI/IRI construction and file-path diagnostics |
| [`serde`] | 1 | URI/IRI strings, ancestor depths, UUIDs, trait-object serialization |
| [`url`] | 2.5.8 | Checked `Url` conversions; WHATWG/IDNA normalization |
| [`uriparse`] | 0.6.4 | Checked `URI` conversions; ASCII only |

- `serde`, `url`, `iri-string`, and `fluent-uri` support `no_std` with allocation.
  The other integrations above enable `std`.
- `clap`, `miette`, and `serde` extend enabled components; with defaults disabled,
  select the desired types too (e.g. `features = ["uri", "clap"]`).
- URI/IRI scheme enums come from [`known-schemes`] 0.2.
- [`uuid`] 1.27 conversions are built into `dogma-uuid`. Its `v1`, `v3`–`v8`,
  `arbitrary`, `atomic`, `borsh`, `bytemuck`, and `zerocopy` features forward to
  upstream; generation and these extra integrations use `uuid::Uuid`.

## 👨‍💻 Development

```bash
git clone https://github.com/dogmatists/dogma.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dogmatists/dogma.rs&text=Dogma.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dogmatists/dogma.rs&title=Dogma.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dogmatists/dogma.rs&t=Dogma.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dogmatists/dogma.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dogmatists/dogma.rs)

[Rust]: https://rust-lang.org
[changelog]: CHANGES.md
[feature flags]: #feature-flags
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[`dogma`]: https://docs.rs/dogma
[`dogma-traits`]: https://docs.rs/dogma-traits
[`dogma-path`]: https://docs.rs/dogma-path
[`dogma-uri`]: https://docs.rs/dogma-uri
[`dogma-uuid`]: https://docs.rs/dogma-uuid

[`camino`]: https://crates.io/crates/camino
[`clap`]: https://crates.io/crates/clap
[`email-address`]: https://crates.io/crates/email_address
[`fluent-uri`]: https://crates.io/crates/fluent-uri
[`iref`]: https://crates.io/crates/iref
[`iri-string`]: https://crates.io/crates/iri-string
[`known-schemes`]: https://crates.io/crates/known-schemes
[`miette`]: https://crates.io/crates/miette
[`serde`]: https://crates.io/crates/serde
[`url`]: https://crates.io/crates/url
[`uriparse`]: https://crates.io/crates/uriparse
[`uuid`]: https://crates.io/crates/uuid

[`Collection`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.Collection.html
[`CollectionMut`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.CollectionMut.html
[`Countable`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.Countable.html
[`Labeled`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.Labeled.html
[`MaybeCountable`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.MaybeCountable.html
[`MaybeLabeled`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.MaybeLabeled.html
[`MaybeNamed`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.MaybeNamed.html
[`Named`]: https://docs.rs/dogma-traits/latest/dogma_traits/trait.Named.html
