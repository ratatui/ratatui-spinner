//! Horizontal and vertical Linear galleries

use super::*;

/// Every Linear symbol pair with a restrained color and display label
const STYLES: [(LinearStyle, Color, &str); 6] = [
    (LinearStyle::Classic, Color::White, "Classic"),
    (LinearStyle::Square, Color::Cyan, "Square"),
    (LinearStyle::Diamond, Color::Magenta, "Diamond"),
    (LinearStyle::Bar, Color::LightGreen, "Bar"),
    (LinearStyle::Braille, Color::Yellow, "Braille"),
    (LinearStyle::Arrow, Color::LightRed, "Arrow"),
];

/// Compares every style in forwards and backwards horizontal flows
pub(super) fn render_horizontal(frame: &mut Frame, tick: u64) {
    let rows: [Rect; 7] = Layout::vertical([1; 7]).areas(frame.area());
    let columns = Layout::horizontal([7, 7, 8, 9]).spacing(1);
    let [style, symbols, forwards, backwards] = columns.areas(rows[0]);
    frame.render_widget(span!(Modifier::BOLD; "Style"), style);
    frame.render_widget(span!(Modifier::BOLD; "Symbols"), symbols);
    frame.render_widget(span!(Modifier::BOLD; "Forwards"), forwards);
    frame.render_widget(span!(Modifier::BOLD; "Backwards"), backwards);

    for ((style, color, name), row) in zip(STYLES, &rows[1..]) {
        let forwards = LinearSpinner::new(tick)
            .total_slots(8)
            .lit_slots(2)
            .ticks_per_step(1)
            .flow(Flow::Forwards)
            .linear_style(style)
            .active_color(color);
        let backwards = LinearSpinner::new(tick)
            .total_slots(9)
            .lit_slots(2)
            .ticks_per_step(1)
            .flow(Flow::Backwards)
            .linear_style(style)
            .active_color(color);
        let (on, off) = style.symbols(Direction::Horizontal);
        let [name_area, symbols_area, forwards_area, backwards_area] = columns.areas(*row);

        frame.render_widget(span!(color; name), name_area);
        frame.render_widget(span!(color; "{on}{off}"), symbols_area);
        frame.render_widget(forwards, forwards_area);
        frame.render_widget(backwards, backwards_area);
    }
}

/// Compares both vertical flows while keeping symbols and columns aligned
pub(super) fn render_vertical(frame: &mut Frame, tick: u64) {
    let columns = Layout::horizontal([7, 6, 7, 5, 7, 5]).spacing(2);
    let tiles: [Rect; 6] = columns.areas(frame.area());

    for ((style, color, name), tile) in zip(STYLES, tiles) {
        let forwards = LinearSpinner::new(tick)
            .direction(Direction::Vertical)
            .total_slots(5)
            .ticks_per_step(2)
            .flow(Flow::Forwards)
            .linear_style(style)
            .active_color(color);
        let backwards = LinearSpinner::new(tick)
            .direction(Direction::Vertical)
            .total_slots(5)
            .ticks_per_step(2)
            .flow(Flow::Backwards)
            .linear_style(style)
            .active_color(color);
        let (on, off) = style.symbols(Direction::Vertical);
        let rows = Layout::vertical([1, 1, 1, 5]).areas(tile);
        let [heading, symbols_area, arrows, spinners] = rows;
        let tracks = Layout::horizontal([1, 1]).spacing(1);
        let [up_arrow, down_arrow] = tracks.areas(arrows);
        let [forwards_area, backwards_area] = tracks.areas(spinners);

        frame.render_widget(span!(color; name), heading);
        frame.render_widget(span!(color; "{on}{off}"), symbols_area);
        frame.render_widget("↑", up_arrow);
        frame.render_widget("↓", down_arrow);
        frame.render_widget(forwards, forwards_area);
        frame.render_widget(backwards, backwards_area);
    }
}
