# Project files
- Don't ask to examine parent directories, stick to the project directory.
- When updating `AGENTS.md`, keep in mind that the file is meant to especially
  benefit lesser models than yourself, such as GPT-5.6 Sol, Terra, and Luna;
  but they also have smaller context windows, so be terse and token-efficient.
- Don't update `README.md` casually: beneficial additions require significant
  judgment and discernment, possibly beyond your capabilities. Longer and more
  detailed does *not* in fact equal better, because humans are not LLMs!
- Any documentation you might wish to add to the README likely better belongs
  as inline comments to the modules and/or types in question, where you are
  allowed elaborated length. For example, in Rust code `rustdoc` coverage of
  every public symbol is a worthy goal, but brevity and quality matter.
- Keep to an 80-column wordwrap in comments and Markdown files, where feasible.

# Rust code
- Don't ever directly read the contents of `Cargo.lock`, it can very large.
- Our current MSRV is Rust 1.97; update and enforce everywhere as needed.
- Our crates collect their default features under an `all` feature, and
  their `[features]` should start with the line `default = ["all", "std"]`.
- Our crates are meant to always be buildable with `#![no_std]` and hence
  always export an explicit `std` feature flag. Some of our lower-level crates
  may also have an explicit `alloc` feature, but for many crates it's not
  possible to do anything useful without heap allocations and they hence
  implicitly assume and omit such a feature.
- All references to `std`, `alloc`, and `core` types should always use
  qualified names or explicit, least-power imports. For example, prefer
  `core::error::Error` and `alloc::string::String` over `std` analogs.
- After making changes to a crate, as a last step run `cargo doc` on it.

# Workspace
- The sole crate is `lib/dogma` (Rust 2021). Root `Cargo.toml` owns shared
  package metadata; keep release versions aligned with `VERSION`.
- Under `lib/dogma/src/`: `traits/` holds collection/count/name/label traits;
  `enums/` and `structs/` wrap `iri-string` and `known-schemes`; `path/` holds
  `AncestorPath` and `FromPathError`.
- Keep feature dependencies in the crate manifest, module/re-export gates in
  `lib.rs` and group modules, and `features.rs::FEATURES` consistent.
- Preserve `#![deny(unsafe_code)]`. Gate filesystem/network APIs on `std`
  and optional integrations on their named features.
- `all` includes `serde`; `--all-features` also enables opt-in integrations
  (`camino`, `clap`, `miette`). Check affected features with defaults disabled.
- URI types currently alias IRI types; `Iri::to_uri()` currently just clones.

# Checks
For Rust changes, run from the repository root:

```sh
cargo fmt --all -- --check
cargo test -p dogma
cargo test -p dogma --all-features
cargo check -p dogma --no-default-features
cargo check -p dogma --no-default-features --features all
cargo doc -p dogma --all-features --no-deps
```

- Unit tests are in `lib/dogma/src/path/ancestor_path.rs`; README examples
  run as doctests via `lib.rs`. Cover POSIX and Windows inputs for path changes.
- `.github/workflows/ci.yaml` builds, builds examples, and tests on Ubuntu
  and Windows.
- Stable rustfmt warns about the nightly-only `imports_granularity` setting.
