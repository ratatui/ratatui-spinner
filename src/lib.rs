//! Customizable animated spinner widgets for [Ratatui] applications.
//!
//! Spinners do not store or read time. Each spinner value renders one animation frame for the tick
//! passed to its constructor. The application owns the clock and redraw schedule. Spinner values
//! and references implement Ratatui's [`Widget`](ratatui::widgets::Widget) trait.
//!
//! # Rendering
//!
//! Store a wrapping tick counter in application state. Advance it on an animation timer, request a
//! redraw, and construct the spinner from that tick while drawing:
//!
//! ```no_run
//! use ratatui::layout::Rect;
//! use ratatui::style::Color;
//! use ratatui::Frame;
//! use ratatui_spinner::FluxSpinner;
//!
//! #[derive(Default)]
//! struct App {
//!     spinner_tick: u64,
//! }
//!
//! impl App {
//!     // Call this when the application's animation timer fires.
//!     fn advance_animation(&mut self) {
//!         self.spinner_tick = self.spinner_tick.wrapping_add(1);
//!     }
//! }
//!
//! fn render(frame: &mut Frame, area: Rect, app: &App) {
//!     let spinner = FluxSpinner::new(app.spinner_tick).color(Color::Cyan);
//!     frame.render_widget(spinner, area);
//! }
//! ```
//!
//! A redraw caused by input or another widget can reuse the same tick; the spinner advances only
//! when the application changes the value. The runnable
//! [`spinner`](https://github.com/ratatui/ratatui-spinner/blob/main/examples/spinner.rs) example
//! includes the event loop.
//!
//! # Animation speed
//!
//! The visible speed combines the application's timer interval with each spinner's
//! `ticks_per_step` setting. At a 50 ms timer interval, `ticks_per_step(1)` advances every 50 ms
//! and `ticks_per_step(4)` holds each animation step for 200 ms. Zero is clamped to one.
//!
//! - [`LinearSpinner::ticks_per_step`] defaults to 3.
//! - [`BarSpinner::ticks_per_step`], [`FluxSpinner::ticks_per_step`],
//!   [`CircleSpinner::ticks_per_step`], [`SquareSpinner::ticks_per_step`], and
//!   [`RectSpinner::ticks_per_step`] default to 1.
//!
//! # Choosing a spinner
//!
//! | Spinner           | Best suited to                          | Shape and motion                  | Main choices                                                  |
//! |-------------------|-----------------------------------------|-----------------------------------|---------------------------------------------------------------|
//! | [`LinearSpinner`] | Inline activity and narrow side columns | Scrolling row or bouncing column  | [`Direction`], [`Flow`], [`LinearStyle`], slot counts         |
//! | [`BarSpinner`]    | Prominent progress-like activity        | Horizontal or vertical moving arc | [`BarMotion`], [`BarOrientation`], [`BarStyle`], [`BarTrack`] |
//! | [`FluxSpinner`]   | Compact indicators and repeating fields | Animated glyph sequence or wave   | [`FluxFrames`], dimensions, phase, [`Spin`]                   |
//! | [`CircleSpinner`] | Circular activity indicators            | Rotating braille arc              | Radius, arc length, [`Spin`]                                  |
//! | [`SquareSpinner`] | Square activity indicators              | Rotating braille arc              | Size, [`Centre`], [`Spin`]                                    |
//! | [`RectSpinner`]   | Configurable braille-ring indicators    | Rotating braille arc              | [`RectShape`], [`Centre`], [`Spin`]                           |
//!
//! The constructors share the same tick value, while each spinner exposes builders for its own
//! shape and motion:
//!
//! ```
//! use ratatui::style::Color;
//! use ratatui_spinner::{
//!     BarMotion, BarSpinner, Centre, CircleSpinner, Direction, FluxFrames, FluxSpinner,
//!     LinearSpinner, RectShape, RectSpinner, Spin, SquareSpinner,
//! };
//!
//! let tick = 42;
//!
//! let linear = LinearSpinner::new(tick)
//!     .direction(Direction::Vertical)
//!     .active_color(Color::Cyan);
//! let bar = BarSpinner::new(tick)
//!     .motion(BarMotion::Loop)
//!     .arc_color(Color::Yellow);
//! let flux = FluxSpinner::new(tick)
//!     .frames(FluxFrames::ORBIT)
//!     .color(Color::LightBlue);
//! let circle = CircleSpinner::new(tick)
//!     .radius(5)
//!     .spin(Spin::CounterClockwise);
//! let square = SquareSpinner::new(tick).size(3).centre(Centre::Empty);
//! let rectangle = RectSpinner::new(tick).shape(RectShape::Square(4));
//!
//! # let _ = (linear, bar, flux, circle, square, rectangle);
//! ```
//!
//! `SquareSpinner` takes a size directly; `RectSpinner` takes a [`RectShape`]. Their rendering
//! algorithms and dimensions differ. [`RectShape::Square`] is the only rectangle shape.
//!
//! # Sizing and layout
//!
//! Ratatui clips a spinner to the area supplied to `render_widget`. Most spinners also expose their
//! intrinsic dimensions for layout calculations:
//!
//! - [`LinearSpinner`] uses `total_slots` columns in horizontal mode or rows in vertical mode.
//!   [`LinearStyle::columns_per_slot`] reports the width of each symbol.
//! - [`FluxSpinner::char_size`], [`CircleSpinner::char_size`], and [`SquareSpinner::char_size`]
//!   return exact dimensions in terminal cells.
//! - [`BarSpinner::char_size`] returns `None` while its width is the default `0`, because widget
//!   rendering then fills the available width. A positive [`BarSpinner::width`] makes its size
//!   explicit.
//! - `RectSpinner` currently has no public size-query method; allocate space for the selected
//!   [`RectShape`] and let Ratatui clip any excess.
//!
//! Every spinner has builders for an optional [`Block`](ratatui::widgets::Block) and a base
//! [`Style`](ratatui::style::Style). Bar, Flux, Circle, Square, and Rect spinners also have
//! alignment builders. A block's borders reduce the area available to the spinner body.
//!
//! # Embedding frames in text widgets
//!
//! Frame size is intrinsic for [`LinearSpinner`], [`FluxSpinner`], [`CircleSpinner`],
//! [`SquareSpinner`], and [`RectSpinner`]. Each converts by value or reference into
//! [`Text`](ratatui::text::Text), so it can be passed directly to a table
//! [`Cell`](ratatui::widgets::Cell), [`Paragraph`](ratatui::widgets::Paragraph), or another
//! text-accepting widget:
//!
//! ```
//! use ratatui::text::Line;
//! use ratatui::widgets::{Cell, Paragraph};
//! use ratatui_spinner::FluxSpinner;
//!
//! let spinner = FluxSpinner::new(3).width(12);
//! let cell = Cell::from(&spinner);
//! let paragraph = Paragraph::new(spinner.clone());
//!
//! let mut lines = vec![Line::from("Indexing")];
//! lines.extend(spinner.to_lines());
//! let combined = Cell::from(lines);
//! # let _ = (cell, paragraph, combined);
//! ```
//!
//! An automatically sized `BarSpinner` has no width until widget rendering. Text conversion
//! therefore takes explicit dimensions through [`BarSpinner::to_lines`] or
//! [`BarSpinner::to_text`]:
//!
//! ```
//! use ratatui::widgets::Cell;
//! use ratatui_spinner::BarSpinner;
//!
//! let spinner = BarSpinner::new(3);
//! let cell = Cell::from(spinner.to_text(20, 1));
//! # let _ = cell;
//! ```
//!
//! The
//! [`table_embed`](https://github.com/ratatui/ratatui-spinner/blob/main/examples/table_embed.rs)
//! example includes each conversion path.
//!
//! # API reference
//!
//! Builder defaults, dimensions, and focused examples are documented with their types:
//!
//! - [`LinearSpinner`] with [`Direction`], [`Flow`], and [`LinearStyle`]
//! - [`BarSpinner`] with [`BarMotion`], [`BarOrientation`], [`BarStyle`], and [`BarTrack`]
//! - [`FluxSpinner`] with the [`FluxFrames`] presets
//! - [`CircleSpinner`]
//! - [`SquareSpinner`]
//! - [`RectSpinner`] with [`RectShape`], [`Centre`], and [`Spin`]
//!
//! [Ratatui]: https://github.com/ratatui/ratatui

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
mod macros;

mod bar_spinner;
mod circle_spinner;
mod flux_spinner;
mod linear_spinner;
mod rect_spinner;
mod square_spinner;

pub use bar_spinner::{BarMotion, BarOrientation, BarSpinner, BarStyle, BarTrack};
pub use circle_spinner::CircleSpinner;
pub use flux_spinner::{FluxFrames, FluxSpinner};
pub use linear_spinner::{Direction, Flow, LinearSpinner, LinearStyle};
pub use rect_spinner::{Centre, RectShape, RectSpinner, Spin};
pub use square_spinner::SquareSpinner;
