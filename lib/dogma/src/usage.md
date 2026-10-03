Common naming, labeling, and collection traits, plus IRI and URI types.

## Naming an object

Implement `Named` when every value has a name. The returned `Cow` can own a
computed name:

```rust
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
```

## Optional names

Implement `MaybeNamed` when a name may be absent. An existing name can be
borrowed without allocating:

```rust
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
```
