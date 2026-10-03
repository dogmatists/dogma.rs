# Validated URI migration

Migration record for URI-01 and URI-02. The validated URI API is now active;
the contracts below describe the implemented API. Publish in a breaking release.
Native filesystem coverage passed on Windows (Rust 1.97 and stable) in
[CI run 37115521648](https://github.com/dogmatists/dogma.rs/actions/runs/37115521648)
before activation.
The audited baseline is dogma 0.2.2, iri-string 0.7.8, and known-schemes 0.2.0.
Rust source paths below are relative to `lib/dogma/src/`.

## Pre-migration coupling

- `enums/uri.rs` aliases `Uri` to `Iri`, inheriting IRI constructors, variants,
  methods, and trait implementations. Unicode input is therefore accepted.
- `enums/uri_error.rs` aliases both `UriError` and `UriResult` to IRI types.
  `structs/uri_authority.rs` likewise aliases `UriAuthority` to `IriAuthority`.
- `enums/iri.rs::to_uri()` returns a clone typed as `Uri`. Replacing the alias
  makes that implementation fail to compile.
- `enums/integrations/clap.rs` is included by the IRI module. Its
  `UriValueParser` alias has `TypedValueParser::Value = Iri<'static>`.
- `uri` enables `iri`, which enables `alloc`. `enums.rs`, `structs.rs`, and
  `lib.rs` gate the exports; `features.rs` already reports both identifiers.
- Serde is forwarded to dependencies but has no dogma identifier impls.
  Miette derives diagnostics on IRI errors; Camino currently serves path APIs.
  Upstream `IriScheme` aliases `UriScheme`, which can remain shared.

## Target contracts

- `Uri<'a>` is an enum with `Borrowed(&'a UriStr)` and `Owned(UriString)`.
  Upstream validation enforces ASCII URI syntax, including a required scheme
  and optional fragment. Parsing rejects Unicode, malformed escapes, and
  relative references; it preserves spelling without normalization or decoding.
- `TryFrom<&'a str>` borrows, `FromStr` owns, and
  `TryFrom<alloc::string::String>` consumes the allocation. `From<&UriStr>` and
  `From<&UriString>` borrow; `From<UriString>` moves into `Uri<'static>`.
  `into_owned()` copies borrowed data and moves owned data without reallocating.
- `as_uri_str()`, `as_iri_str()`, `as_str()`, and `AsRef<str>` expose borrowed
  views. Preserve the existing component accessor names and encoded output.
  Scheme recognition keeps the IRI implementation's case-insensitive behavior
  and complete known-scheme lookup; `scheme()` returns `UriScheme`.
- Equality, ordering, and hashing use the exact string, independently of
  ownership. `Display` emits that string; `Debug` identifies the URI variant.
  Retain `Uri::to_uri()` as an identity clone for existing callers.
- `UriError` becomes distinct, retaining the corresponding `IriError` variant
  names and payloads. Invalid borrowed input maps to `Invalid(None)`; an owned
  creation error retains its string. Path variants remain `std`-gated.
  Use URI-specific display text and Miette diagnostic codes.
  `UriResult<T>` becomes `core::result::Result<T, UriError>`.
- `UriAuthority` becomes a wrapper constructed from `&Uri`, with the existing
  accessors, traits, and `TryFrom` error `()`. Share component storage and
  socket resolution with `IriAuthority`; references borrow the original URI,
  not a temporary `Iri` wrapper. Network APIs remain `std`-gated.
- `From<Uri<'a>> for Iri<'a>` preserves ownership and spelling using upstream
  conversions. `From<&'a Uri<'_>> for Iri<'a>` borrows. Strict
  `TryFrom<&'a Iri<'_>> for Uri<'a>` rejects non-ASCII input without encoding.
  A `From<&Iri>` impl would conflict with that `TryFrom` via the blanket impl.
- `Iri::to_uri(&self) -> Uri<'_>` is the explicit encoding operation. Borrow
  ASCII input through `IriStr::as_uri()`; otherwise use `encode_to_uri()` and
  `ToDedicatedString` to produce an owned `UriString`. Preserve the source,
  existing escapes, case, and dot segments. Encode Unicode as UTF-8 percent
  escapes in every component, including registered-name hosts. This is not
  IDNA/Punycode conversion and does not add hostname resolution support.
- URI filesystem construction delegates to the existing IRI path conversion,
  then encodes into an owned URI. Preserve relative/non-Unicode/special-prefix
  rejection. `to_path()` and `try_to_path()` delegate through an IRI view;
  retain the existing `IriToPathError` return type and decoding rules.
- `UriValueParser` becomes distinct, takes `&[UriScheme]`, and returns
  `Uri<'static>`. Preserve scheme filtering and Clap error kinds while rejecting
  Unicode URI input. Gate it on both `uri` and `clap`.

## Completed migration sequence

Build replacement types in crate-private staging submodules of the existing
URI modules. Keep public aliases until activation, so preparatory commits can
compile without incomplete public APIs. Refer explicitly to staged types in
their impls; implementing traits against today's aliases can conflict with IRI
impls. Each step includes focused regression coverage.

1. **Errors:** stage `UriError` and `UriResult`, upstream error conversions,
   formatting, `core::error::Error`, and gated Miette diagnostics. Check invalid
   borrowed/owned input payloads and the `std`-gated variants.
2. **Construction:** stage the URI enum, strict string/upstream constructors,
   and borrowed views. Check ASCII, Unicode in each component, malformed
   escapes, relative references, and borrowed pointer identity.
3. **Ownership:** add `into_owned()`, cloning, and the identity `to_uri()`.
   Check borrowed input can be dropped after conversion and owned allocations
   are reused when consumed.
4. **Value traits:** add formatting and ownership-independent comparison/hash
   impls. Check borrowed/owned pairs and ordered/hash collection keys.
5. **Components:** add scheme and raw component accessors. Check empty versus
   absent components, original spelling, path segments, and all known schemes.
6. **Strict bridges:** add URI-to-IRI conversions and checked IRI borrowing.
   Check lifetime/ownership preservation and rejection without modifying input.
7. **Encoding (URI-02):** stage a crate-private IRI-to-URI adapter with the
   encoding contract above. Check ASCII borrowing, Unicode in every component,
   mixed existing escapes, host behavior, and unchanged input. This must precede
   activation: the current cloning `Iri::to_uri()` cannot return a distinct URI.
8. **Authority:** stage the URI authority wrapper. Share construction from a
   scheme and `AuthorityComponents` to preserve source lifetimes.
   Check accessors, absent authority, default/explicit ports, and IPv6 literals.
9. **Filesystem:** add `std`-gated URI path construction and decoding adapters.
   Check POSIX and native Windows drive/UNC round trips, Unicode encoding,
   reserved characters, and existing rejection/error behavior.
10. **CLI:** stage the URI value parser alongside the IRI parser. Assert the
    associated output type, scheme allowlists, invalid UTF-8, and Unicode
    rejection with defaults disabled under `uri,clap`.
11. **Activation:** replace the URI/error/authority/parser aliases together and
    route `Iri::to_uri()` through the tested adapter. Re-export staged types
    from existing groups, remove staging-only scaffolding, and add public API
    smoke tests. Include caller migration rustdoc and a breaking-change entry in
    `CHANGES.md`; publish the replacement in a breaking release.

## Caller migration at activation

- For Unicode text, parse an `Iri`, then call `to_uri()` when ASCII is needed.
  `https://example.com/café` becomes `https://example.com/caf%C3%A9`; direct URI
  parsing now rejects the former. Call `into_owned()` for a `'static` result.
- Replace implicit URI/IRI assignments with `Iri::from(uri)` or the strict
  `Uri::try_from(&iri)`. Compare across types via conversion or `as_str()`.
- Construct URI variants from `UriStr`/`UriString`, rather than IRI strings.
  For existing upstream IRI values, validate their string views or wrap them in
  `Iri` and use the encoding operation. Match URI errors as `UriError`.
- Use `UriAuthority` with `Uri` and `IriAuthority` with `Iri`.
  Retrieve Clap values as `Uri<'static>` when using `UriValueParser`.

## Feature and verification boundaries

Keep `default = ["all", "std"]`, `uri = ["iri"]`, and the current allocation
requirements. Every staged module and export must use its identifier gate;
filesystem/network and optional integrations keep their existing gates.
Preserve `no_std` and `deny(unsafe_code)`. Identifier dependencies are optional
and activated by `iri`. Serde support was added after URI activation:
identifiers use plain strings and deserialization validates into owned values.

For implementation commits, run the root checks in `AGENTS.md`, with the
documentation build last. Additionally run Rust 1.97 library tests with defaults
disabled for `iri` and `uri`; check `uri` on `thumbv7em-none-eabihf`. Exercise
affected combinations of each identifier with `std`, `clap`, `miette`, `camino`,
and `serde` independently. Use `--lib` for minimal-feature tests until DOC-02
fixes the unrelated README doctests. The filesystem step also requires native
Windows CI evidence; a cross-compiled check alone does not verify path behavior.

Upstream contracts: [borrowed IRI conversion][iri-str],
[owned IRI conversion][iri-string], and [URI views][uri-str].

[iri-str]: https://docs.rs/iri-string/0.7.8/iri_string/types/type.IriStr.html
[iri-string]:
  https://docs.rs/iri-string/0.7.8/iri_string/types/type.IriString.html
[uri-str]: https://docs.rs/iri-string/0.7.8/iri_string/types/type.UriStr.html
