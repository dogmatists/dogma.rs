# Enhancement backlog

Recheck the code before starting an item.
Paths below are relative to the repository root. Implementations live in
`lib/dogma-{traits,path,uri,uuid}/src/`; `lib/dogma` is the umbrella.

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
  including historical `CHANGES.md` headings. Update VERSION, workspace package
  metadata, and internal dependency versions deliberately; preserve history
  and add a new changelog entry for a new release. Let Cargo refresh its lockfile
  through normal commands. Verify the bump behavior on fixtures before using it
  for a release.

## Documentation and verification

- [ ] **DOC-04: Document public API behavior incrementally.**
  Prioritize identifier ownership, encoded versus decoded components, identifier
  path normalization, platform differences, feature requirements, and conversion
  errors. Extend rustdoc in `lib/dogma-uri/src/`,
  `lib/dogma-path/src/`, and `lib/dogma-traits/src/` in small patches. Keep
  examples usable directly from each component crate and through the umbrella.

- [ ] **QA-01: Expand behavior coverage as APIs are improved.**
  Add focused tests for file conversions, Serde, and CLI parsing.
  Cover remaining native POSIX/Windows conversions and string parsing.
  Useful property tests include identifier/path encoding round trips.
  Put behavior tests in the owning crate; keep umbrella tests focused on
  re-exports and feature forwarding. Test real examples when adding them;
  `lib/dogma/examples/` currently contains only `.gitkeep`.

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
