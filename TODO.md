# Enhancement backlog

Recheck the code before starting an item.
Paths below are relative to the repository root. Implementations live in
`lib/dogma-{traits,path,uri,uuid}/src/`; `lib/dogma` is the umbrella.
Sections are ordered by priority, and so are the items within each section.
Items marked **Breaking** belong in the next minor release, with a
`CHANGES.md` entry.

## Working method

- Follow `AGENTS.md`. Implement one narrowly scoped, independently verifiable
  improvement per request. Split larger goals below into smaller steps before
  coding; an unchecked goal is not a request to implement its whole section.
- Each change should be suitable for one atomic commit. Create commits only
  when explicitly requested.
- Add meaningful regression coverage alongside behavior changes. After
  verification, remove fully completed items and retain only remaining substeps
  for partial work.
- Preserve `no_std`, `deny(unsafe_code)`, and feature gates. Prefer module/type
  rustdoc over expanding the README. The target MSRV is Rust 1.97.

## Packaging, tooling, and CI

- [ ] **PKG-02: Give each crate its own crates.io metadata.**
  `keywords = []` in every crate. `readme = true` makes every component
  crate ship the umbrella README (`cargo add dogma`). `UNLICENSE` is not
  packaged. The umbrella ships `examples/.gitkeep`, and `tests/.gitkeep` is
  now redundant. Add keywords per crate, short per-crate READMEs, and the
  license file.

- [ ] **REL-01: Make version bumping target explicit fields.**
  `Rakefile` globally replaces the old version in all matching tracked files,
  including historical `CHANGES.md` headings. Update VERSION, workspace package
  metadata, and internal dependency versions deliberately; preserve history
  and add a new changelog entry for a new release. Let Cargo refresh its lockfile
  through normal commands. Verify the bump behavior on fixtures before using it
  for a release.

- [ ] **CI-03: Add API regression and maintenance tooling.**
  Run `cargo-semver-checks` for each crate against its last release. Update
  the pinned action versions, and add a dependency cache. The `Makefile`
  `check` target runs only `cargo test`: align it with the checks in
  `AGENTS.md` or remove it.

- [ ] **PKG-03: Decide what the umbrella's `unstable` feature is for.**
  `lib/dogma/Cargo.toml` defines `unstable = []`, but no code uses it. Either
  document it as reserved or remove it in the next breaking release.

## API features and ergonomics

- [ ] **IDN-01: Add string ergonomics to `Iri` and `Uri`.**
  Equality, ordering, and hashing already match `str`, so implement
  `Borrow<str>` (allows `HashMap<Iri, _>::get("...")`). Also add
  `PartialEq<str>`/`PartialEq<&str>`, cross-type `PartialEq` between `Iri`
  and `Uri`, and `into_string()` with `From<_> for String`. Add one trait
  family per commit.

- [ ] **IDN-02: Re-export the upstream types used in public signatures.**
  The `Iri`/`Uri` variants and `From` impls expose `iri_string::types::*`.
  `dogma-uuid` converts to and from `uuid::Uuid`. Callers must therefore add
  matching direct dependencies. Re-export these crates or types, and note in
  rustdoc that bumping them is a breaking change.

- [ ] **IDN-03: Share the scheme lookup and URI accessors.**
  `Iri::scheme()` and `Uri::scheme()` duplicate a linear scan of
  `IriScheme::ALL`, and both allocate for unknown schemes. The scan works
  around a parsing gap in `known-schemes` 0.2.0/0.2.1. `try_to_path` and
  `authority()` call it on every use. Share one helper and compare
  `scheme_str()` directly for `file`. Report the gap upstream, and drop the
  workaround once a fixed version becomes the floor. Most `Uri` accessors
  duplicate `Iri` ones; consider having them delegate.

- [ ] **API-06: Resolve the overlap between `Countable` and `Collection`.**
  `countable.rs` has `FIXME: change to len()?`. A type implementing both
  traits has an ambiguous `is_empty`/`is_nonempty`, the same problem the
  `maybe_` prefixes solved. Decide whether to rename, deprecate, or document
  the split, record the decision in rustdoc, and remove the FIXME.

- [ ] **PATH-01: Make `AncestorPath` `Copy`.**
  It wraps a `NonZeroUsize`. Also add `From<AncestorPath>` for
  `NonZeroUsize` and `usize`. Keep the blanket `From<&T>` impl for
  compatibility.

- [ ] **API-07: Replace the `()` error of the authority `TryFrom` impls.**
  **Breaking.** `TryFrom<&Iri>` for `IriAuthority` and `TryFrom<&Uri>` for
  `UriAuthority` fail with `()`, which does not implement `Error` and does
  not work well with `?`. Use a named zero-sized error, or point callers to
  `authority()`.

- [ ] **API-08: Mark pure accessors `#[must_use]`.**
  Clippy's `must_use_candidate` reports 65 public functions whose results are
  easy to discard by mistake. Review them, add `#[must_use]` where ignoring
  the result is a bug, and consider enabling the lint in `[workspace.lints]`.

## Documentation

- [ ] **DOC-04: Document public API behavior incrementally.**
  Prioritize identifier ownership, encoded versus decoded components, identifier
  path normalization, platform differences, feature requirements, and conversion
  errors. Extend rustdoc in `lib/dogma-uri/src/`,
  `lib/dogma-path/src/`, and `lib/dogma-traits/src/` in small patches. Keep
  examples usable directly from each component crate and through the umbrella.
  The thinnest docs are the `dogma-traits` trait and method docs and the
  Serde impls on trait objects.

- [ ] **DOC-07: Repair the links in `CHANGES.md`.**
  Reference links point to the removed `dogma/enums/...` pages. `IriAuthority`
  and `IriError` (0.1.5) have no link definitions. Link historical entries
  to versioned docs.rs pages, and add an `Unreleased` section.

- [ ] **DOC-08: Retire `doc/uri-migration.md`.**
  It records a finished migration, still calls `dogma-uuid` a placeholder,
  and repeats the `Uri` rustdoc and `CHANGES.md`. Shrink it to a short
  historical note, or delete it once 0.3 callers have migrated.

## Verification

- [ ] **QA-01: Expand behavior coverage as APIs are improved.**
  Add focused tests for file conversions, Serde, and CLI parsing.
  Cover remaining native POSIX/Windows conversions and string parsing.
  Useful property tests include identifier/path encoding round trips.
  Put behavior tests in the owning crate; keep umbrella tests focused on
  re-exports and feature forwarding. Test real examples when adding them;
  `lib/dogma/examples/` currently contains only `.gitkeep`. Specific gaps:
  - `IriValueParser` tests cover only error messages; mirror the
    `UriValueParser` success, filtering, and UTF-8 tests.
  - In `lib/dogma-path/src/ancestor_path.rs`, raw strings such as
    `r"..\\.."` and `r"\\\\server\\share"` contain doubled backslashes, so
    they test repeated separators rather than the intended input.
    `from_str_errors` also repeats the `../file` case.
  - `Collection` impls for arrays, slices, and `VecDeque`/`LinkedList` are
    only covered indirectly.
  - The umbrella re-exports components with glob imports, so a future name
    clash between components would go unnoticed. Add a test that names every
    top-level export.

## Validation for implementation work

Run from the repository root, as required by `AGENTS.md`:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo test --workspace --all-features
cargo check --workspace --no-default-features
cargo check --workspace --no-default-features --features all
cargo doc --workspace --all-features --no-deps
```

Run additional relevant checks before the final documentation build. For IRI
changes, useful targeted commands are:

```sh
cargo +1.97.0 test -p dogma-uri --no-default-features --features iri
cargo +1.97.0 check -p dogma-uri --no-default-features --features iri \
  --target thumbv7em-none-eabihf
```

Install that target for Rust 1.97 if needed. Check affected crates individually
to avoid workspace feature unification hiding missing gates. Also check `uri`
and affected interop features with defaults disabled, including their forwarding
through `dogma`, when changing APIs or gates. For lint/documentation cleanup
tasks, use:

```sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```

For dependency changes, also run:

```sh
cargo +nightly update -Z direct-minimal-versions
cargo +nightly test --workspace --all-features
git checkout Cargo.lock
```
