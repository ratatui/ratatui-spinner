# ratatui-spinner

[![MIT License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

Customizable, stateless spinner widgets for
[Ratatui](https://github.com/ratatui/ratatui) applications.

![Overview of the ratatui-spinner widgets](examples/vhs/generated/overview.gif)

## Widgets

| Widget                            | Best suited to                          | Shape and motion                  | Notable options                      |
| --------------------------------- | --------------------------------------- | --------------------------------- | ------------------------------------ |
| [`LinearSpinner`](#linearspinner) | Inline activity and narrow side columns | Scrolling row or bouncing column  | Orientation, flow, symbols, length   |
| [`BarSpinner`](#barspinner)       | Prominent progress-like activity        | Horizontal or vertical moving arc | Motion, thickness, track, symbols    |
| [`FluxSpinner`](#fluxspinner)     | Compact indicators and repeating fields | Animated glyph sequence or wave   | Frames, dimensions, phase, direction |
| [`CircleSpinner`](#circlespinner) | Circular activity indicators            | Rotating braille arc              | Radius, arc length, direction        |
| [`SquareSpinner`](#squarespinner) | Square activity indicators              | Rotating braille arc              | Size, centre, direction              |
| [`RectSpinner`](#rectspinner)     | Configurable braille-ring indicators    | Rotating braille arc              | Shape, centre, direction             |

## Installation

The fork from `tui-spinner` to `ratatui-spinner` is currently unpublished. Use
the Git repository while the API is under development:

```toml
[dependencies]
ratatui-spinner = { git = "https://github.com/ratatui/ratatui-spinner" }
```

For a local checkout, use a path dependency instead:

```toml
[dependencies]
ratatui-spinner = { path = "../ratatui-spinner" }
```

Every spinner is stateless. Pass the current `tick: u64` when constructing it,
then increment the tick at the cadence your application uses for animation:

```rust
struct App {
    tick: u64,
}

fn update(app: &mut App) {
    app.tick = app.tick.wrapping_add(1);
}
```

Render a spinner directly with `Frame::render_widget`:

```rust
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui_spinner::FluxSpinner;

fn render(frame: &mut Frame, area: Rect, tick: u64) {
    let spinner = FluxSpinner::new(tick).color(Color::Cyan);
    frame.render_widget(spinner, area);
}
```

## `LinearSpinner`

`LinearSpinner` renders a moving window of symbols across a row or a single
symbol bouncing in a column. It works well beside status text and in layouts
where the available width or height is known.

```text
Horizontal: ●●· → ·●● → ··● → ●·· → …
Vertical:   ●   →   ·   →   ·   →   ·
            ·       ●       ·       ●
            ·       ·       ●       ·
```

### Horizontal linear spinners

![Horizontal LinearSpinner styles and directions][linear-h]

### Vertical linear spinners

![Vertical LinearSpinner styles and directions][linear-v]

```rust
use ratatui::style::Color;
use ratatui_spinner::{Direction, Flow, LinearSpinner, LinearStyle};

let horizontal = LinearSpinner::new(tick)
    .total_slots(8)
    .lit_slots(2)
    .flow(Flow::Backwards)
    .linear_style(LinearStyle::Diamond)
    .active_color(Color::Cyan);

let vertical = LinearSpinner::new(tick)
    .direction(Direction::Vertical)
    .total_slots(5)
    .ticks_per_step(4)
    .linear_style(LinearStyle::Arrow);
```

| Builder                     | Default       | Purpose                                |
| --------------------------- | ------------- | -------------------------------------- |
| `direction(Direction)`      | `Horizontal`  | Select a row or column                 |
| `flow(Flow)`                | `Forwards`    | Reverse the animation direction        |
| `linear_style(LinearStyle)` | `Classic`     | Select the symbol pair                 |
| `total_slots(n)`            | `3`           | Set the length of the spinner          |
| `lit_slots(n)`              | `2`           | Set the horizontal moving-window width |
| `ticks_per_step(n)`         | `3`           | Hold each animation step for `n` ticks |
| `active_color(color)`       | `White`       | Style active symbols                   |
| `inactive_color(color)`     | `DarkGray`    | Style inactive symbols                 |
| `style(style)`              | default style | Set the base widget style              |
| `block(block)`              | none          | Render inside a `Block`                |

`LinearStyle` provides `Classic` (`●·`), `Square` (`■□`), `Diamond` (`◆◇`),
`Bar` (`▰▱`), `Braille` (`⣿⠀`), and `Arrow` (`▶▷` horizontally or `▼▽`
vertically).

| `Direction`  | Motion                                 |
| ------------ | -------------------------------------- |
| `Horizontal` | Scroll a window across a row (default) |
| `Vertical`   | Bounce one symbol up and down a column |

| `Flow`      | Motion                                         |
| ----------- | ---------------------------------------------- |
| `Forwards`  | Left-to-right or top-to-bottom first (default) |
| `Backwards` | Right-to-left or bottom-to-top first           |

| `LinearStyle` | Active    | Inactive  |
| ------------- | --------- | --------- |
| `Classic`     | `●`       | `·`       |
| `Square`      | `■`       | `□`       |
| `Diamond`     | `◆`       | `◇`       |
| `Bar`         | `▰`       | `▱`       |
| `Braille`     | `⣿`       | `⠀`       |
| `Arrow`       | `▶` / `▼` | `▷` / `▽` |

## `BarSpinner`

`BarSpinner` moves a bright arc over a dim track. In addition to the familiar
bounce, it can loop across the track, squeeze inward from both sides, or
radiate from the centre. Its orientation and thickness make it suitable for
both single-line indicators and larger loading treatments.
The braille style tapers each arc edge through a density ramp, giving it a soft glow.

### Horizontal bars

![Horizontal BarSpinner styles and options][bar-h]

### Vertical bars

![Vertical BarSpinner styles and options][bar-v]

```rust
use ratatui::style::Color;
use ratatui_spinner::{BarMotion, BarOrientation, BarSpinner, BarStyle, Spin};

let horizontal = BarSpinner::new(tick)
    .width(20)
    .arc_width(4)
    .motion(BarMotion::Radiate)
    .bar_style(BarStyle::Block)
    .arc_color(Color::Magenta);

let vertical = BarSpinner::new(tick)
    .orientation(BarOrientation::Vertical)
    .height(8)
    .thickness(2)
    .spin(Spin::CounterClockwise);
```

`width(0)`, the default, fills the available width when rendered as a widget.

| Builder                    | Default            | Purpose                                          |
| -------------------------- | ------------------ | ------------------------------------------------ |
| `width(n)`                 | `0` (fill area)    | Set a fixed horizontal length                    |
| `height(n)`                | `1`                | Set the vertical length or horizontal bar height |
| `orientation(orientation)` | `Horizontal`       | Select the motion axis                           |
| `thickness(n)`             | `0` (axis default) | Set the cross-axis thickness                     |
| `arc_width(n)`             | `0` (automatic)    | Set the length of the bright arc                 |
| `motion(motion)`           | `Bounce`           | Set the edge behavior                            |
| `spin(spin)`               | `Clockwise`        | Select the initial or continuous direction       |
| `bar_style(style)`         | `Braille`          | Select the arc and track glyphs                  |
| `track(track)`             | `Rail`             | Select the braille track treatment               |
| `fade_width(n)`            | `3`                | Set the braille density-ramp width               |
| `arc_char(byte)`           | `0xFF` (`⣿`)       | Set a custom braille arc byte                    |
| `ticks_per_step(n)`        | `1`                | Hold each animation step for `n` ticks           |
| `arc_color(color)`         | `Cyan`             | Style the moving arc                             |
| `dim_color(color)`         | `DarkGray`         | Style the track                                  |
| `with_colors(arc, dim)`    | —                  | Set both colors together                         |
| `alignment(alignment)`     | `Left`             | Align a fixed-size bar in its area               |
| `style(style)`             | default style      | Set the base widget style                        |
| `block(block)`             | none               | Render inside a `Block`                          |

`BarStyle` includes `Braille`, `Block`, `Shade`, `Dot`, `Diamond`, `Square`,
`Star`, `Heart`, `Arrow`, `Circle`, `Spark`, `Cross`, `Progress`, `Thick`,
`Wave`, and `Pip`.

| `BarMotion` | Edge behavior                                    |
| ----------- | ------------------------------------------------ |
| `Bounce`    | Reverse at each edge (default)                   |
| `Loop`      | Exit one edge and re-enter from the other        |
| `Squeeze`   | Move two arcs inward from the edges, then bounce |
| `Radiate`   | Move two arcs outward from the centre, then wrap |

| `BarStyle` | Arc | Track | `BarStyle` | Arc | Track |
| ---------- | --- | ----- | ---------- | --- | ----- |
| `Braille`  | `⣿` | `⣀`   | `Arrow`    | `▶` | `▷`   |
| `Block`    | `█` | `░`   | `Circle`   | `◉` | `○`   |
| `Shade`    | `▓` | `░`   | `Spark`    | `✦` | `✧`   |
| `Dot`      | `●` | `·`   | `Cross`    | `✚` | `✛`   |
| `Diamond`  | `◆` | `◇`   | `Progress` | `▰` | `▱`   |
| `Square`   | `■` | `□`   | `Thick`    | `━` | `─`   |
| `Star`     | `★` | `☆`   | `Wave`     | `≈` | `˜`   |
| `Heart`    | `♥` | `♡`   | `Pip`      | `▪` | `·`   |

Only `BarStyle::Braille` uses `fade_width`, `arc_char`, and `BarTrack`. The symbol styles use the
fixed pairs above.

| `BarTrack`   | Byte   | Glyph | Treatment                     |
| ------------ | ------ | ----- | ----------------------------- |
| `Rail`       | `0xC0` | `⣀`   | Bottom-dot baseline (default) |
| `Full`       | `0xFF` | `⣿`   | Full-density dim track        |
| `Empty`      | `0x00` | `⠀`   | Invisible track               |
| `Custom(u8)` | any    | —     | User-defined braille byte     |

Common configurations are also available as constructors:

```rust
let zed = BarSpinner::zed(tick); // 1 row, cyan, rail track
let claude = BarSpinner::claude(tick); // 2 rows, orange, rail track
let minimal = BarSpinner::minimal(tick); // 1 row, white, empty track
let solid = BarSpinner::solid(tick); // 1 row, cyan, full track, no fade
```

## `FluxSpinner`

`FluxSpinner` cycles through a sequence of characters. A `1 × 1` spinner is a
compact status indicator; increasing its width or height creates a field of
phase-shifted glyphs that reads as a travelling wave.

![FluxSpinner frame presets and configuration](examples/vhs/generated/flux.gif)

```rust
use ratatui::style::Color;
use ratatui_spinner::{FluxFrames, FluxSpinner, Spin};

let spinner = FluxSpinner::new(tick)
    .frames(FluxFrames::ORBIT)
    .width(8)
    .height(2)
    .phase_step(1)
    .spin(Spin::CounterClockwise)
    .color(Color::LightGreen);
```

| Builder                | Default               | Purpose                                         |
| ---------------------- | --------------------- | ----------------------------------------------- |
| `frames(frames)`       | `FluxFrames::BRAILLE` | Set a static character sequence                 |
| `width(n)`             | `1`                   | Set the number of columns                       |
| `height(n)`            | `1`                   | Set the number of rows                          |
| `phase_step(n)`        | `1`                   | Offset adjacent cells within the frame sequence |
| `spin(spin)`           | `Clockwise`           | Reverse frame and wave direction                |
| `ticks_per_step(n)`    | `1`                   | Hold each frame for `n` ticks                   |
| `color(color)`         | `Cyan`                | Style the glyphs                                |
| `alignment(alignment)` | `Left`                | Align the spinner in its area                   |
| `style(style)`         | default style         | Set the base widget style                       |
| `block(block)`         | none                  | Render inside a `Block`                         |

`FluxFrames` provides `BRAILLE`, `ORBIT`, `CLASSIC`, `LINE`, `BLOCK`, `ARC`,
`CLOCK`, `MOON`, `TRIANGLES`, `PULSE`, `BOUNCE`, `HALF`, `SQUARE`, `DICE`,
`BAR`, `CORNERS`, `CIRCLE_FILL`, `PISTON`, `STAR`, `PAIR`, and `DIAMOND`.
Custom frames use the same API:

```rust
let custom = FluxSpinner::new(tick).frames(&['◐', '◓', '◑', '◒']);
```

| Preset        | Glyphs                | Frames | Description                              |
| ------------- | --------------------- | ------ | ---------------------------------------- |
| `BRAILLE`     | `⣾ ⣷ ⣯ ⣟ ⡿ ⢿ ⣽ ⣻`     | 8      | Full cell with one dot missing (default) |
| `ORBIT`       | `⠁ ⠈ ⠐ ⠠ ⢀ ⡀ ⠄ ⠂`     | 8      | Single dot orbiting                      |
| `CLASSIC`     | `⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏` | 10     | Classic braille spinner                  |
| `LINE`        | `│ ╱ ─ ╲`             | 4      | Rotating line                            |
| `BLOCK`       | `▖ ▘ ▝ ▗`             | 4      | Quarter-block rotation                   |
| `ARC`         | `◜ ◝ ◞ ◟`             | 4      | Quarter-arc rotation                     |
| `CLOCK`       | `◷ ◶ ◵ ◴`             | 4      | Quarter-circle pie slice                 |
| `MOON`        | `◓ ◑ ◒ ◐`             | 4      | Half-circle moon phase                   |
| `TRIANGLES`   | `▲ ▶ ▼ ◀`             | 4      | Filled triangle rotation                 |
| `PULSE`       | `⣀ ⣤ ⣶ ⣾ ⣿ ⣾ ⣶ ⣤`     | 8      | Braille fill pulse                       |
| `BOUNCE`      | `⠉ ⠒ ⣀ ⠒`             | 4      | Braille row bouncing                     |
| `HALF`        | `▀ ▐ ▄ ▌`             | 4      | Half-block rotation                      |
| `SQUARE`      | `◰ ◳ ◲ ◱`             | 4      | Filled square quadrant                   |
| `DICE`        | `⚀ ⚁ ⚂ ⚃ ⚄ ⚅`         | 6      | Dice faces                               |
| `BAR`         | `▁ ▂ ▃ ▄ ▅ ▆ ▇ █`     | 8      | Growing bar                              |
| `CORNERS`     | `┌ ┐ ┘ └`             | 4      | Rotating box corners                     |
| `CIRCLE_FILL` | `○ ◔ ◑ ◕ ●`           | 5      | Filling circle                           |
| `PISTON`      | `▁ ▃ ▅ ▇ █ ▇ ▅ ▃`     | 8      | Bouncing bar                             |
| `STAR`        | `✶ ✷ ✸ ✹`             | 4      | Star density ramp                        |
| `PAIR`        | `⠉ ⠘ ⠰ ⢠ ⣀ ⡄ ⠆ ⠃`     | 8      | Two dots rotating together               |
| `DIAMOND`     | `◇ ◈ ◆ ◈`             | 4      | Diamond pulse                            |

Adjacent cells advance by `phase_step` frames, producing a wave when either dimension is greater
than one:

```text
width = 6, phase_step = 1, Clockwise
⣾⣷⣯⣟⡿⢿   (tick 0)
⣷⣯⣟⡿⢿⣽   (tick 1)
⣯⣟⡿⢿⣽⣻   (tick 2)
```

`Spin::CounterClockwise` reverses both the frame sequence and the wave direction.

## `CircleSpinner`

`CircleSpinner` moves a bright braille-dot arc around a circular dim ring computed with the
midpoint circle algorithm. Its radius is expressed in braille dots; `char_size()` returns the
terminal columns and rows needed by the result.

![CircleSpinner radii, arc lengths, and directions](examples/vhs/generated/circle.gif)

```rust
use ratatui::style::Color;
use ratatui_spinner::{CircleSpinner, Spin};

let spinner = CircleSpinner::new(tick)
    .radius(6)
    .arc_len(8)
    .spin(Spin::CounterClockwise)
    .arc_color(Color::Yellow);
```

| Builder                | Default                 | Purpose                               |
| ---------------------- | ----------------------- | ------------------------------------- |
| `radius(n)`            | `4`                     | Set the circle radius in braille dots |
| `arc_len(n)`           | `0` (automatic quarter) | Set the arc length in dots            |
| `spin(spin)`           | `Clockwise`             | Set the rotation direction            |
| `ticks_per_step(n)`    | `1`                     | Hold each arc position for `n` ticks  |
| `arc_color(color)`     | `White`                 | Style the bright arc                  |
| `dim_color(color)`     | `DarkGray`              | Style the dim ring                    |
| `alignment(alignment)` | `Left`                  | Align the circle in its area          |
| `style(style)`         | default style           | Set the base widget style             |
| `block(block)`         | none                    | Render inside a `Block`               |

## `SquareSpinner`

`SquareSpinner` moves a bright braille-dot arc around a square. The centre can
be filled to give the spinner visual weight or left empty so only the ring
remains.

![SquareSpinner sizes, centres, and directions](examples/vhs/generated/square.gif)

```rust
use ratatui::style::Color;
use ratatui_spinner::{Centre, Spin, SquareSpinner};

let spinner = SquareSpinner::new(tick)
    .size(4)
    .centre(Centre::Empty)
    .spin(Spin::Clockwise)
    .arc_color(Color::LightRed);
```

| Builder                | Default       | Purpose                              |
| ---------------------- | ------------- | ------------------------------------ |
| `size(n)`              | `2`           | Set the square size from 2 through 8 |
| `centre(centre)`       | `Filled`      | Select a filled or empty centre      |
| `spin(spin)`           | `Clockwise`   | Set the rotation direction           |
| `ticks_per_step(n)`    | `1`           | Hold each arc position for `n` ticks |
| `arc_color(color)`     | `White`       | Style the bright arc                 |
| `dim_color(color)`     | `DarkGray`    | Style the dim ring and centre        |
| `alignment(alignment)` | `Left`        | Align the square in its area         |
| `style(style)`         | default style | Set the base widget style            |
| `block(block)`         | none          | Render inside a `Block`              |

`SquareSpinner` is a convenience wrapper around `RectSpinner`. `RectSpinner`
is the more general entry point for new code.

## `RectSpinner`

`RectSpinner` is the configurable foundation for braille-ring spinners. The
current `RectShape` variant is `Square(n)`, with filled and empty centre
treatments and clockwise or counter-clockwise motion.

![RectSpinner shapes, centres, and directions](examples/vhs/generated/rect.gif)

```rust
use ratatui::style::Color;
use ratatui_spinner::{Centre, RectShape, RectSpinner, Spin};

let spinner = RectSpinner::new(tick)
    .shape(RectShape::Square(4))
    .centre(Centre::Empty)
    .spin(Spin::CounterClockwise)
    .outer_color(Color::LightCyan);
```

| Builder                | Default                | Purpose                              |
| ---------------------- | ---------------------- | ------------------------------------ |
| `shape(shape)`         | `RectShape::Square(2)` | Select the shape and size            |
| `centre(centre)`       | `Filled`               | Select a filled or empty centre      |
| `spin(spin)`           | `Clockwise`            | Set the rotation direction           |
| `ticks_per_step(n)`    | `1`                    | Hold each arc position for `n` ticks |
| `outer_color(color)`   | `Cyan`                 | Style the moving outer arc           |
| `inner_color(color)`   | `DarkGray`             | Style the centre                     |
| `alignment(alignment)` | `Left`                 | Align the spinner in its area        |
| `style(style)`         | default style          | Set the base widget style            |
| `block(block)`         | none                   | Render inside a `Block`              |

`RectShape::Square(n)` produces square output with a size from 2 through 8.

`SquareSpinner`, `RectSpinner`, and `CircleSpinner` share `Spin`: `Clockwise` is the default, and
`CounterClockwise` reverses the arc. The square and rectangle spinners also share `Centre`:
`Filled` draws a solid interior by default, while `Empty` leaves only the moving ring.

## Embedding spinners in text widgets

Every spinner can be converted into `Text`, so it can be used in a table
`Cell`, a `Paragraph`, a `ListItem`, or another widget that accepts text:

```rust
use ratatui::widgets::{Cell, Paragraph};
use ratatui_spinner::FluxSpinner;

let spinner = FluxSpinner::new(tick).width(8);
let cell = Cell::from(&spinner);
let owned_cell = Cell::from(spinner);
let paragraph = Paragraph::new(FluxSpinner::new(tick));
```

Use `to_lines()` or `to_text()` when combining a spinner with other content.
`LinearSpinner`, `FluxSpinner`, `CircleSpinner`, `SquareSpinner`, and
`RectSpinner` know their intrinsic size. `BarSpinner` accepts explicit width
and height arguments for text conversion because its widget width can be
automatic.

```rust
use ratatui::text::Line;
use ratatui::widgets::Cell;
use ratatui_spinner::FluxSpinner;

let spinner = FluxSpinner::new(tick).width(12);
let mut lines = vec![Line::from("The cell content")];
lines.extend(spinner.to_lines());
let cell = Cell::from(lines);
```

![Spinners converted to text and embedded in table cells](examples/vhs/generated/text.gif)

See [`examples/table_embed.rs`](examples/table_embed.rs) for a complete example.

## Running the examples

```sh
cargo run --example linear_spinner
cargo run --example bar_spinner
cargo run --example flux_spinner
cargo run --example circle_spinner
cargo run --example square_spinner
cargo run --example spinner
cargo run --example table_embed
```

The compact README galleries share one example with a scene argument:

```sh
cargo run --example readme -- --help
cargo run --example readme -- overview
```

The gallery source lives in [`examples/readme/`](examples/readme/). Its tapes and generated GIFs use
the existing [`examples/vhs/`](examples/vhs/) layout.

## Provenance

This project is a fork of Sorin Albu-Irimies's
[`tui-spinner`](https://github.com/sorinirimies/tui-spinner), maintained by the
Ratatui project. The original code remains MIT-licensed, with its authorship
and commit history preserved.

## Development

Run `just check-all` for formatting, Clippy, unit tests, doc tests, and
documentation checks. Formatting uses `cargo +nightly fmt` with Ratatui
conventions. Builds and tests use stable Rust; documentation uses nightly to
check the docs.rs configuration.

Run `just vhs-all` to regenerate and losslessly optimize every gallery, or record one scene
directly. GIF files are tracked with Git LFS; use `just lfs-pull` after a fresh clone.

```sh
just vhs-tape linear-horizontal
```

The recording commands require [VHS](https://github.com/charmbracelet/vhs).

Run `just package` to verify the crate archive without publishing and `just changelog-preview` to
preview release notes. See [AGENTS.md](AGENTS.md) for the underlying Cargo commands and repository
guidance.

Publication remains disabled while the `tui-spinner` fork is prepared as
`ratatui-spinner`.

## License

MIT — see [LICENSE](LICENSE).

[bar-h]: examples/vhs/generated/bar-horizontal.gif
[bar-v]: examples/vhs/generated/bar-vertical.gif
[linear-h]: examples/vhs/generated/linear-horizontal.gif
[linear-v]: examples/vhs/generated/linear-vertical.gif
