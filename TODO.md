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
- The current enhancement focus is IRIs/URIs. For the next atomic IRI fix,
  start with IRI-07. PATH-01 is the highest-priority non-IRI correctness fix.
- Add meaningful regression coverage alongside behavior changes. After
  verification, remove fully completed items and retain only remaining substeps
  for partial work.
- Preserve `no_std`, `deny(unsafe_code)`, and feature gates. Prefer module/type
  rustdoc over expanding the README. The target MSRV is Rust 1.97.

## IRI/URI correctness

- [ ] **IRI-07: Implement native Windows file-path conversion.**
  Work in `enums/iri.rs`; preserve the non-Unicode-path policy.
  - [ ] Implement UNC path construction and conversion with authority handling.
  - [ ] Verify native Windows runtime tests on CI, including drive round trips,
    reserved characters, special-prefix rejection, and non-Unicode rejection.

## Identifier APIs and serialization

- [ ] **URI-01: Design and introduce a genuinely validated URI type.**
  `enums/uri.rs` aliases `Uri` to `Iri`, so parsing
  `https://example.com/café` as a `Uri` currently succeeds. Use the upstream
  `iri_string::types::{UriStr,UriString}` validation model for ASCII URIs.
  First plan small, compiling migration steps: construction, ownership,
  comparison, conversion, feature gates, and integrations. Audit the aliases
  in `enums/uri_error.rs`, `structs/uri_authority.rs`, and
  `enums/integrations/clap.rs`; the URI parser must produce the URI type.
  Alias replacement changes the public API and needs a documented migration.

- [ ] **URI-02: Implement real IRI-to-URI conversion.**
  `enums/iri.rs::to_uri()` only clones. Build on URI-01's type/conversion
  contract and upstream conversion facilities. For example,
  `https://example.com/café` must produce an ASCII URI ending in
  `caf%C3%A9`. Cover already-ASCII values, Unicode, existing percent escapes,
  and authority components; document host conversion rules. Preserve the
  source IRI and avoid double encoding.

- [ ] **API-01: Add `AsRef<str>` for `Iri`.**
  Location: `enums/iri.rs`. Expose the existing zero-copy string view through
  the standard trait for interoperability with generic string APIs.

- [ ] **API-02: Centralize access to the underlying validated IRI.**
  Location: `enums/iri.rs`. Consider an `as_iri_str()` accessor or
  `AsRef<IriStr>`, then delegate repeated component-access matches through it.
  Separate the public accessor addition from a broader internal refactor.

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
  round trips and invalid values. Numeric input handling depends on PATH-01.

## Paths and collection traits

- [ ] **PATH-01: Return an error for zero ancestor depth instead of panicking.**
  `path/ancestor_path.rs::TryFrom<usize>` calls
  `NonZeroUsize::new(0).unwrap()` in its error branch, so
  `AncestorPath::try_from(0usize)` always panics. Its `NonZeroUsize` error type
  also misrepresents the failure. Delegate to `NonZeroUsize::try_from` with its
  error type, or introduce an appropriate depth error. Cover 0, 1, and large
  representable depths without trying to allocate their formatted paths.

- [ ] **PATH-02: Stream ancestor-path formatting.**
  `path/ancestor_path.rs::Display` allocates an intermediate string with
  `"../".repeat(depth)`. Write components directly to the formatter and
  propagate formatting errors. Preserve canonical output, including the
  trailing slash.

- [ ] **COLL-01: Support custom hashers in collection traits.**
  `traits/collection.rs` and `traits/collection_mut.rs` implement traits only
  for default-hasher `HashMap`/`HashSet`. Generalize over the hasher parameter.
  A map using `BuildHasherDefault<DefaultHasher>` currently fails a
  `Collection` bound. Check both immutable and mutable trait implementations.

- [ ] **COLL-02: Remove unnecessary collection bounds.**
  In the same two files, keep only bounds actually required by delegated
  `len`, `is_empty`, and `clear` methods. Audit `Ord`, `Eq`, and `Hash` bounds
  on heap/tree/hash collections; generic read-only operations should not need
  insertion-related bounds.

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

- [ ] **META-01: Align MSRV declarations with Rust 1.97.**
  Root `Cargo.toml` and the README still advertise 1.70, while `AGENTS.md`
  specifies 1.97. Update the manifest and the README badge/prerequisite
  narrowly. Existing code uses newer APIs, including const `Option::unwrap`
  and `ErrorKind::InvalidFilename`.

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

- [ ] **DOC-03: Fix existing rustdoc warnings.**
  `path/from_path_error.rs` has six unresolved links to `Path`, `Utf8Path`,
  and `AncestorPath`, including implementation anchors. Resolve links correctly
  under optional features. `structs/iri_authority.rs` has two bare RFC URLs;
  make them proper links. The all-feature doc build reports eight warnings.

- [ ] **DOC-04: Document public API behavior incrementally.**
  Prioritize identifier ownership, encoded versus decoded components, path
  normalization, platform differences, feature requirements, and conversion
  errors. Extend rustdoc in `enums/`, `structs/`, and `path/` in small patches.
  Explain the collection-trait contracts, including `CollectionMut::clear`.

- [ ] **QA-01: Expand behavior coverage as APIs are improved.**
  Add focused tests for file conversions, Serde, and CLI parsing.
  Cover native POSIX/Windows path conversions as well as string parsing.
  Useful property tests include bounded ancestor-depth round trips and
  identifier/path encoding round trips. Test real examples when adding them;
  `lib/dogma/examples/` currently contains only `.gitkeep`.

- [ ] **QA-03: Enforce formatting, Clippy, and clean documentation in CI.**
  First address META-01 and DOC-03. Strict Clippy currently fails; beyond stale
  MSRV diagnostics, findings include redundant closures, needless `Ok(...?)`,
  `ok_or_else(|| ())`, decimal `from_str_radix`, and manual separator matching.
  Clean these up in bounded patches before enabling warning-as-error checks.
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
