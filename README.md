# crafty_novels

A library for converting text formats.
Intended for converting Minecraft: Java Edition books to HTML,
but it exposes traits necessary to implement arbitrary formats.

## Supported formats

### Import

- [Stendhal](https://modrinth.com/mod/stendhal) exports

### Export

- HTML

## Implementations

### [crafty_novels_cli](./crafty_novels_cli)

Currently, only for testing the lib.

In the future, it will handle file/stdin parsing, file/stdout export, and possibly a TUI (similar to the [GUI](#GUI)) using [Ratatui](https://ratatui.rs/).

### crafty_novels_gui

Not yet implemented. Will likely use [Iced](https://iced.rs/) to implement a simple and user-friendly file picker and format selector interface.

## Roadmap

- ~~Basic Stendhal syntax parsing~~
  - ~~Frontmatter parsing~~
- ~~Basic HTML export~~
  - ~~Frontmatter export~~
- Apply [API best practices](https://rust-lang.github.io/api-guidelines/checklist.html)
  - More/better documentation for public items
    - Crate level docs\
      [C-CRATE-DOC](https://rust-lang.github.io/api-guidelines/documentation.html#crate-level-docs-are-thorough-and-include-examples-c-crate-doc)
    - All items have examples\
      [C-EXAMPLE](https://rust-lang.github.io/api-guidelines/documentation.html#all-items-have-a-rustdoc-example-c-example)
    - Examples use `?`, not `unwrap`\
      [C-QUESTION-MARK](https://rust-lang.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark)
    - Documentation of errors `Result` functions and other failures (including for traits)\
      [C-FAILURE](https://rust-lang.github.io/api-guidelines/documentation.html#function-docs-include-error-panic-and-safety-considerations-c-failure)
    - Frequently use of links in doc comments\
      [C-LINK](https://rust-lang.github.io/api-guidelines/documentation.html#prose-contains-hyperlinks-to-relevant-things-c-link)
    - Release notes and Git tags\
      [C-RELNOTES](https://rust-lang.github.io/api-guidelines/documentation.html#release-notes-document-all-significant-changes-c-relnotes)
    - Use of `pub(crate)` and `#[doc(hidden)]` to hide unhelpful implementation details from appearing in Rustdoc\
      [C-HIDDEN](https://rust-lang.github.io/api-guidelines/documentation.html#rustdoc-does-not-show-unhelpful-implementation-details-c-hidden)
  - Expose intermediary steps
  - Implement common traits
    - [C-COMMON-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits)
    - [C-SEND-SYNC](https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync)
    - [C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err)
    - [C-SERDE](https://rust-lang.github.io/api-guidelines/interoperability.html#data-structures-implement-serdes-serialize-deserialize-c-serde)
    - [C-NUM-FMT](https://rust-lang.github.io/api-guidelines/interoperability.html#binary-number-types-provide-hex-octal-binary-formatting-c-num-fmt)
    - [C-RW-VALUE](https://rust-lang.github.io/api-guidelines/interoperability.html#generic-readerwriter-functions-take-r-read-and-w-write-by-value-c-rw-value)
    - [C-DEBUG](https://rust-lang.github.io/api-guidelines/debuggability.html#all-public-types-implement-debug-c-debug)
      - Debug representations are never empty
        (ex. `assert_eq!(format!("{:?}", ""), r#""""#)`)\
        [C-DEBUG-NONEMPTY](https://rust-lang.github.io/api-guidelines/debuggability.html#debug-representation-is-never-empty-c-debug-nonempty)
  - Predicable, dependable, and type-safe code
    - Bind conversions to the most specific form of a data
      (ex. `str::from_utf8`, not `<&[u8]>::to_string`)\
      [C-CONV-SPECIFIC](https://rust-lang.github.io/api-guidelines/predictability.html#conversions-live-on-the-most-specific-type-involved-c-conv-specific)
    - Arguments convey meaning and requirements through types
      - Ex. `Widget::new(Small, Round)`, not `Widget::new(true, false)` for clarity\
        [C-CUSTOM-TYPE](https://rust-lang.github.io/api-guidelines/type-safety.html)
      - Ex. `fn foo(a: Ascii)` instead of `fn foo(a: u8) -> Result<_, InvalidAscii>` (or worse, a `foo` that panics)\
        [C-VALIDATE#Static Enforcement](https://rust-lang.github.io/api-guidelines/dependability.html#static-enforcement)
    - Use newtypes to limit the scope of a function signature's promises and hide implementation details\
      [C-NEWTYPE-HIDE](https://rust-lang.github.io/api-guidelines/future-proofing.html#newtypes-encapsulate-implementation-details-c-newtype-hide)
      - Use of `-> impl Trait` maybe be used instead, but comes with some limitations like not ensuring `Debug`
    - Use `derives` instead of trait bounds for generic structs where possible\
      [C-STRUCT-BOUNDS](https://rust-lang.github.io/api-guidelines/future-proofing.html#newtypes-encapsulate-implementation-details-c-newtype-hide)
  - Flexible code
    - When encountering errors
      -- whether returned, like `String::from_utf8` stopping at bad UTF-8,
      or ignored like `HashMap::insert` overwriting values at the key --
      return intermediary work,
      like the point in an array that a missing item should have been
      or the point in a buffer where a bad byte was found.\
      [C-INTERMEDIATE](https://rust-lang.github.io/api-guidelines/flexibility.html#functions-expose-intermediate-results-to-avoid-duplicate-work-c-intermediate)
    - Prefer private fields (using getters/setters) to avoid committing to an API\
      [C-STRUCT-PRIVATE](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
  - Consider changing to a more permissive license
    (like the LGPL or MIT & Apache 2.0)
    for wider compatibility\
    [C-PERMISSIVE](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive)
- CLI
  - TUI
- GUI

## License

crafty_novels is in no way affiliated with Microsoft, Mojang, Minecraft, Stendhal, or NebSpacefarer. All trademarks belong to their respective owners.

crafty_novels is licensed under the GNU Affero General Public License version 3, or (at your option) any later version.
You should have received a copy of the GNU Affero General Public License along with crafty_novels, found in [LICENSE](./LICENSE).
If not, see \<[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
