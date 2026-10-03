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
- Add meaningful regression coverage alongside behavior changes. After
  verification, remove fully completed items and retain only remaining substeps
  for partial work.
- Preserve `no_std`, `deny(unsafe_code)`, and feature gates. Prefer module/type
  rustdoc over expanding the README. The target MSRV is Rust 1.97.

## Features, compatibility, and release tooling

- [ ] **REL-01: Make version bumping target explicit fields.**
  `Rakefile` globally replaces the old version in all matching tracked files,
  including historical `CHANGES.md` headings. Update VERSION and workspace
  package metadata deliberately, preserve history, and add a new changelog
  entry for a new release. Let Cargo refresh its lockfile through normal
  commands. Verify the bump behavior on fixtures before using it for a release.

## Documentation and verification

- [ ] **DOC-04: Document public API behavior incrementally.**
  Prioritize identifier ownership, encoded versus decoded components, identifier
  path normalization, platform differences, feature requirements, and conversion
  errors. Extend rustdoc in `enums/`, `structs/`, and `path/` in small patches.

- [ ] **QA-01: Expand behavior coverage as APIs are improved.**
  Add focused tests for file conversions, Serde, and CLI parsing.
  Cover remaining native POSIX/Windows conversions and string parsing.
  Useful property tests include identifier/path encoding round trips.
  Test real examples when adding them; `lib/dogma/examples/` currently contains
  only `.gitkeep`.

- [ ] **QA-04: Verify published-package doctests in CI.**
  Test the extracted package as well as the checkout. A successful
  `cargo package` alone does not exercise doctest documentation inputs.

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
