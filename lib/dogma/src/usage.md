Umbrella crate for shared traits, paths, and identifiers.

The component crates are re-exported as namespaces:

- `dogma::traits` is `dogma-traits`: naming, labeling, counting, and collections.
- `dogma::path` is `dogma-path`: ancestor paths and conversion errors.
- `dogma::uri` is `dogma-uri`: validated URIs, IRIs, and authority components.
- `dogma::uuid` is `dogma-uuid`: a placeholder for future UUID support.

Top-level type exports, such as `dogma::Named` and `dogma::Iri`, are also
available. Component crates can be used directly without depending on the
umbrella.

Default features enable all components, Serde support, and `std`. Disable
defaults for `no_std`, then select `all` or individual features (`traits`,
`named`, `labeled`, `countable`, `collection`, `path`, `iri`, `uri`, `uuid`).
The compatibility `alloc` feature enables `path`, as it did before the split.
`std` and optional interop features are forwarded to enabled component crates;
`camino` enables native path support, while `clap` and `miette` extend identifiers.

The naming examples below require the `named` feature, enabled by default.

## Naming an object

Implement `Named` when every value has a name. The returned `Cow` can own a
computed name:

```rust
# #[cfg(feature = "named")]
# {
use dogma::Named;
use std::borrow::Cow;

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
use dogma::MaybeNamed;
use std::borrow::Cow;

struct UserProfile {
    display_name: Option<String>,
}

impl MaybeNamed for UserProfile {
    fn name(&self) -> Option<Cow<'_, str>> {
        self.display_name.as_ref().map(Cow::from)
    }
}

let profile = UserProfile { display_name: Some("Ada".into()) };
assert!(matches!(profile.name(), Some(Cow::Borrowed("Ada"))));
assert_eq!(UserProfile { display_name: None }.name(), None);
# }
```
