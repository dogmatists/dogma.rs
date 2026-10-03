# Enhancement backlog

Recheck the code before starting an item.
Rust source paths below are relative to `lib/dogma/src/`; other paths are
relative to the repository root.

## Working method

- Follow `AGENTS.md`. Implement one narrowly scoped, independently verifiable
  improvement per request. Split larger goals below into smaller steps before
  coding; an unchecked goal is not a request to implement its whole section.
- Each change should be suitable for one atomic commit. Create commits only
  when explicitly requested.
- The current enhancement focus is IRIs/URIs. Continue with URI-01's first
  remaining substep.
- Add meaningful regression coverage alongside behavior changes. After
  verification, remove fully completed items and retain only remaining substeps
  for partial work.
- Preserve `no_std`, `deny(unsafe_code)`, and feature gates. Prefer module/type
  rustdoc over expanding the README. The target MSRV is Rust 1.97.

## Identifier APIs and serialization

- [ ] **URI-01: Introduce a genuinely validated URI type.**
  Follow the [migration plan](doc/uri-migration.md) for contracts, source audit,
  compiling sequence, caller migration, and per-step verification. Prepare
  crate-private replacements before switching public aliases. Each substep is
  a separate atomic change; URI-02's encoder must precede filesystem adapters
  and activation.
  - [ ] Add `std`-gated URI filesystem adapters with native path coverage.
    - [ ] Add path decoding with POSIX regression coverage.
    - [ ] Add native Windows drive/UNC and rejection coverage, and run minimal
      URI filesystem tests in CI.
    - [ ] Verify native Windows CI evidence before activation.
  - [ ] Activate URI/error/authority/parser replacements and `Iri::to_uri()`
    together, with public API tests, migration rustdoc, and a changelog entry.

- [ ] **URI-02: Implement real IRI-to-URI conversion.**
  `enums/iri.rs::to_uri()` only clones. Follow the encoding contract in the
  [migration plan](doc/uri-migration.md): borrow ASCII input, otherwise use
  upstream percent-encoding, including Unicode hostnames without IDNA.
  - [ ] Wire the public method during URI-01 activation, with conversion rustdoc
    and public tests for both ownership forms.

- [ ] **SERDE-01: Support non-`'static` serializable trait objects.**
  The six `impl serde::Serialize for dyn ...` implementations in `traits/`
  implicitly require `'static`: `Named`, `MaybeNamed`, `Labeled`,
  `MaybeLabeled`, `Countable`, and `MaybeCountable`. Passing a borrowed
  `&dyn Named` to a generic `T: serde::Serialize + ?Sized` function fails with
  E0521. Use lifetime-general implementations such as `dyn Named + '_` and
  test genuinely borrowed implementors with defaults disabled, enabling
  `named,serde` or the corresponding trait feature plus `serde`.

- [ ] **SERDE-02: Serialize and deserialize identifier values.**
  The `serde` feature currently does not implement either trait for `Iri`.
  Start with `enums/iri.rs`: use a string wire representation independent of
  ownership, and validate deserialized input. Serialization and deserialization
  can be separate atomic steps. Test both ownership forms, invalid input,
  Unicode, and percent escapes under `no_std` plus `alloc`. Support the distinct
  URI type when URI-01 lands. Keep enum variant tags out of the wire format.

- [ ] **SERDE-03: Define serialization for `AncestorPath`.**
  Location: `path/ancestor_path.rs`. Choose and document a wire representation
  before adding implementations. Preserve the nonzero-depth invariant; test
  round trips and invalid values.

## Features, compatibility, and release tooling

- [ ] **FEAT-01: Make `FEATURES` report the enabled feature set accurately.**
  `features.rs` omits `std`, `alloc`, `serde`, integrations, and umbrella flags
  despite its documentation promising the enabled set. Synchronize it with
  `lib/dogma/Cargo.toml` and add a suitable consistency check.

- [ ] **FEAT-02: Make identifier dependencies optional.**
  `cargo tree -p dogma --no-default-features --edges normal` still includes
  `iri-string` and `known-schemes`. In `lib/dogma/Cargo.toml`, attach these
  dependencies to the identifier features and use weak dependency-feature
  forwarding where appropriate (`dependency?/feature`). Make required Serde
  allocation features explicit instead of relying on transitive activation.
  Verify trait-only builds, especially `named,serde` and `labeled,serde`, as
  well as the identifier/integration matrix.

- [ ] **FEAT-03: Give the `structs` feature meaningful semantics.**
  `structs = []` alone exposes no concrete types and produces an unused
  `structs::*` re-export warning. Define which types this group enables and
  align `lib/dogma/Cargo.toml`, `lib.rs`, and `structs.rs` consistently.

- [ ] **META-02: Correct stale README installation/integration details.**
  Installation examples still select 0.1 rather than the current release line.
  The "all features enabled" example actually selects default features, which
  exclude opt-in integrations. Correct these statements and the `[clap]]` link
  typo without expanding the README. Keep release metadata aligned with VERSION.

- [ ] **REL-01: Make version bumping target explicit fields.**
  `Rakefile` globally replaces the old version in all matching tracked files,
  including historical `CHANGES.md` headings. Update VERSION and workspace
  package metadata deliberately, preserve history, and add a new changelog
  entry for a new release. Let Cargo refresh its lockfile through normal
  commands. Verify the bump behavior on fixtures before using it for a release.

## Documentation and verification

- [ ] **DOC-01: Make doctest inputs work inside the published package.**
  `lib.rs` uses `include_str!("../../../README.md")`, which escapes the packaged
  crate and causes its doctests to fail with a missing-file error. Use
  package-local documentation inputs or module/type examples, preserving
  meaningful doctest coverage.
  Reproduce from the repository root (substitute the current release version):

  ```sh
  cargo package -p dogma
  cargo test --manifest-path target/package/dogma-0.2.2/Cargo.toml --doc
  ```

  Add `--allow-dirty` to `cargo package` when verifying uncommitted changes.

- [ ] **DOC-02: Make doctests feature-aware.**
  `cargo test -p dogma --no-default-features` fails because README examples
  imported by `lib.rs` refer to disabled `Named`/`MaybeNamed` traits. Gate
  feature-specific examples appropriately while retaining enabled-feature
  coverage; do not disable all doctests to make the command pass.

- [ ] **DOC-04: Document public API behavior incrementally.**
  Prioritize identifier ownership, encoded versus decoded components, identifier
  path normalization, platform differences, feature requirements, and conversion
  errors. For `AncestorPath`, document depth constants and relative/absolute
  predicates. Extend rustdoc in `enums/`, `structs/`, and `path/` in small patches.

- [ ] **QA-01: Expand behavior coverage as APIs are improved.**
  Add focused tests for file conversions, Serde, and CLI parsing.
  Cover remaining native POSIX/Windows conversions and string parsing.
  Useful property tests include identifier/path encoding round trips.
  Test real examples when adding them; `lib/dogma/examples/` currently contains
  only `.gitkeep`.

- [ ] **QA-03: Enforce formatting, Clippy, and clean documentation in CI.**
  Add CI checks for rustfmt, Clippy with `-D warnings`, and rustdoc with
  `RUSTDOCFLAGS="-D warnings"`; the current all-feature builds pass locally.
  Stable rustfmt's warning about nightly-only `imports_granularity` is known.

- [ ] **QA-04: Verify published-package doctests in CI.**
  After DOC-01, test the extracted package as well as the checkout. A successful
  `cargo package` alone does not exercise the broken README doctest include.

## Validation for implementation work

Run from the repository root, as required by `AGENTS.md`:

```sh
cargo fmt --all -- --check
cargo test -p dogma
cargo test -p dogma --all-features
cargo check -p dogma --no-default-features
cargo check -p dogma --no-default-features --features all
cargo doc -p dogma --all-features --no-deps
```

Run additional relevant checks before the final documentation build. For IRI
changes, useful targeted commands are:

```sh
cargo +1.97.0 test -p dogma --no-default-features --features iri --lib
cargo +1.97.0 check -p dogma --no-default-features --features iri \
  --target thumbv7em-none-eabihf
```

Install that target for Rust 1.97 if needed. Also check `uri` and affected
integrations with defaults disabled when changing their APIs or gates. For the
lint/documentation cleanup tasks, use:

```sh
cargo clippy -p dogma --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p dogma --all-features --no-deps
```
