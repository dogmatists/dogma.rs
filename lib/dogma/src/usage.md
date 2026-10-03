Umbrella crate for shared traits, paths, and identifiers.

Enabled component crates are re-exported as namespaces:

- `dogma::traits` is `dogma-traits`: naming, labeling, counting, and collections.
- `dogma::path` is `dogma-path`: ancestor paths and conversion errors.
- `dogma::uri` is `dogma-uri`: validated URIs, IRIs, and authority components.
- `dogma::uuid` is `dogma-uuid`: 128-bit UUID values, lossless byte conversions,
  formatting, Serde support, and interoperability with `uuid::Uuid`.

Top-level type exports, such as `dogma::Named`, `dogma::Iri`, and `dogma::Uuid`,
are also available. Component crates can be used directly without depending on
the umbrella. UUID generation and additional UUID interop features are configured
through `dogma-uuid` and accessed through the upstream `uuid::Uuid` type.

## Features

Default features enable all components, Serde support, and `std`. Disable
defaults for `no_std`, then select `all` or individual features (`traits`,
`named`, `labeled`, `countable`, `collection`, `path`, `iri`, `uri`, `uuid`).
The `alloc` feature enables `path` and forwards allocation support to enabled
trait and UUID components. The compatibility `enums` and `structs` features both
enable `iri` and `uri`; they do not introduce additional namespaces.

`std` and `serde` are forwarded to enabled components. Optional interop features
are selected separately from `all`:

- `camino` adds Camino path conversions; use `path,camino` for umbrella exports.
- `clap` and `miette` extend enabled URI/IRI types with CLI parsing and diagnostics.
- `url`, `iri-string`, `fluent-uri`, `email-address`, `uriparse`, and `iref` enable
  both identifier types and conversions with the corresponding foreign crates.
  `email-address` uses `email_address::EmailAddress` for single-mailbox `mailto:`
  conversions.

`camino`, `clap`, `miette`, `email-address`, `uriparse`, and `iref` enable `std`.
`url`, `iri-string`, and `fluent-uri` work with `no_std` and allocation. See the
`dogma-uri` crate docs for conversion, normalization, and ownership rules.

The naming examples below require the `named` feature, enabled by default.

## Naming an object

Implement `Named` when every value has a name. The returned `Cow` can own a
computed name:

```rust
# #[cfg(feature = "named")]
# {
# extern crate alloc;
use alloc::{borrow::Cow, format, string::String};
use dogma::Named;

struct Person {
    first_name: String,
    last_name: String,
}

impl Named for Person {
    fn name(&self) -> Cow<'_, str> {
        format!("{} {}", self.first_name, self.last_name).into()
    }
}

let person = Person { first_name: "Ada".into(), last_name: "Lovelace".into() };
assert_eq!(person.name(), "Ada Lovelace");
# }
```

## Optional names

Implement `MaybeNamed` when a name may be absent. An existing name can be
borrowed without allocating:

```rust
# #[cfg(feature = "named")]
# {
# extern crate alloc;
use alloc::{borrow::Cow, string::String};
use dogma::MaybeNamed;

struct UserProfile {
    display_name: Option<String>,
}

impl MaybeNamed for UserProfile {
    fn maybe_name(&self) -> Option<Cow<'_, str>> {
        self.display_name.as_ref().map(Cow::from)
    }
}

let profile = UserProfile { display_name: Some("Ada".into()) };
assert!(matches!(profile.maybe_name(), Some(Cow::Borrowed("Ada"))));
assert_eq!(UserProfile { display_name: None }.maybe_name(), None);
# }
```
