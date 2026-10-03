# Project files
- Don't ask to examine parent directories, stick to the project directory.
- Read [TODO.md](TODO.md) for remaining enhancements. Make one narrowly scoped,
  atomic change per request; remove completed items after verification.
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
- Rust 2021 crates live in `lib/<crate>/`: `dogma` is the umbrella;
  `dogma-traits` owns collection/count/name/label traits; `dogma-path` owns
  `AncestorPath` and `FromPathError`; `dogma-uri` owns URI/IRI types in flat,
  type-focused source modules; `dogma-uuid` is a placeholder with no UUID API yet.
- Root `Cargo.toml` owns shared metadata/dependencies. Keep package and internal
  dependency versions aligned with `VERSION`; internal defaults stay disabled.
- Component crates are independent of the umbrella. Move behavior tests with
  their implementation; umbrella tests check re-exports and feature forwarding.
- Keep manifests, module gates, and umbrella feature forwarding consistent.
  `dogma::{traits,path,uri,uuid}` re-export crates alongside top-level types.
  Neither the umbrella nor `dogma-uri` has `enums`/`structs` namespaces; those
  feature flags are compatibility aliases. The umbrella's `alloc` enables `path`.
- Preserve `#![deny(unsafe_code)]`. Gate filesystem/network APIs on `std`
  and optional interop on the corresponding features.
- `all` includes `serde` where supported. `dogma-path` owns `camino`;
  `dogma-uri` owns `clap`/`miette`. `dogma-path` and `dogma-uri` assume allocation;
  `dogma-traits` has an explicit `alloc` feature. UUID has only `all`/`std`.
- URI and IRI types are distinct. `Iri::to_uri()` borrows ASCII or encodes Unicode.

# Checks
For Rust changes, run from the repository root:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo test --workspace --all-features
cargo check --workspace --no-default-features
cargo check --workspace --no-default-features --features all
cargo doc --workspace --all-features --no-deps
```

- Also check affected crates individually with defaults disabled: workspace
  feature unification can hide missing gates. Cover POSIX and Windows paths.
- Doctests use package-local inputs. `cargo package --workspace` verifies
  interdependent packages; test extracted doctests with sibling package patches
  as in CI, without depending on published versions of the new crates.
- `.github/workflows/ci.yaml` builds, builds examples, and tests on Ubuntu
  and Windows; it also checks minimal features and a bare-metal `no_std` target.
- Stable rustfmt warns about the nightly-only `imports_granularity` setting.
