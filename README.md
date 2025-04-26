# Dogma.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/dogma)](https://crates.io/crates/dogma)
[![Documentation](https://docs.rs/dogma/badge.svg)](https://docs.rs/dogma/)

Provides the `Named`, `MaybeNamed`, `Labeled`, `MaybeLabeled`, `Collection`,
and `CollectionMut` traits for Rust.

## ✨ Features

- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add dogma
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
dogma = "0.1"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
dogma = { version = "0.1", default-features = false, features = ["traits"] }
```

## 👉 Examples

### Importing the library

```rust
use dogma::*;
```

### Using the `Named` trait

```rust
use dogma::traits::Named;
use std::borrow::Cow;

struct Person {
    pub first_name: String,
    pub last_name: String,
}

impl Named for Person {
    fn name(&self) -> Cow<str> {
        format!("{} {}", self.first_name, self.last_name).into()
    }
}
```

### Using the `MaybeNamed` trait

```rust
use dogma::traits::MaybeNamed;
use std::borrow::Cow;

struct UserProfile {
    pub id: u64,
    pub display_name: Option<String>,
}

impl MaybeNamed for UserProfile {
    fn name(&self) -> Option<Cow<str>> {
        self.display_name.as_ref().map(Cow::from)
    }
}
```

## 📚 Reference

https://docs.rs/dogma/

### Traits

- [`Collection`], [`CollectionMut`]
- [`Countable`], [`MaybeCountable`]
- [`Named`], [`MaybeNamed`]
- [`Labeled`], [`MaybeLabeled`]

### Integrations

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

[feature flags]: https://github.com/dogmatists/dogma.rs/blob/master/lib/dogma/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[`Collection`]: https://docs.rs/dogma/latest/dogma/traits/trait.Collection.html
[`CollectionMut`]: https://docs.rs/dogma/latest/dogma/traits/trait.CollectionMut.html
[`Countable`]: https://docs.rs/dogma/latest/dogma/traits/trait.Countable.html
[`Labeled`]: https://docs.rs/dogma/latest/dogma/traits/trait.Labeled.html
[`MaybeCountable`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeCountable.html
[`MaybeLabeled`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeLabeled.html
[`MaybeNamed`]: https://docs.rs/dogma/latest/dogma/traits/trait.MaybeNamed.html
[`Named`]: https://docs.rs/dogma/latest/dogma/traits/trait.Named.html
