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
  - Frontmatter parsing
- ~~Basic HTML export~~
  - Frontmatter export
- CLI
  - TUI
- GUI

## License

crafty_novels is in no way affiliated with Microsoft, Mojang, Minecraft, Stendhal, or NebSpacefarer. All trademarks belong to their respective owners.

crafty_novels is licensed under the GNU Affero General Public License version 3, or (at your option) any later version.
You should have received a copy of the GNU Affero General Public License along with crafty_novels, found in [LICENSE](./LICENSE).
If not, see \<[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
