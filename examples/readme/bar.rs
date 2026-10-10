//! Horizontal and vertical Bar galleries

use super::*;

/// Every Bar symbol pair with a restrained color and display label
const STYLES: [(BarStyle, Color, &str, &str); 16] = [
    (BarStyle::Braille, Color::White, "Braille", "⣿⣀"),
    (BarStyle::Block, Color::Cyan, "Block", "█░"),
    (BarStyle::Shade, Color::Magenta, "Shade", "▓░"),
    (BarStyle::Dot, Color::LightGreen, "Dot", "●·"),
    (BarStyle::Diamond, Color::Yellow, "Diamond", "◆◇"),
    (BarStyle::Square, Color::LightRed, "Square", "■□"),
    (BarStyle::Star, Color::LightCyan, "Star", "★☆"),
    (BarStyle::Heart, Color::LightMagenta, "Heart", "♥♡"),
    (BarStyle::Arrow, Color::LightGreen, "Arrow", "▶▷"),
    (BarStyle::Circle, Color::Yellow, "Circle", "◉○"),
    (BarStyle::Spark, Color::LightRed, "Spark", "✦✧"),
    (BarStyle::Cross, Color::Cyan, "Cross", "✚✛"),
    (BarStyle::Progress, Color::Magenta, "Progress", "▰▱"),
    (BarStyle::Thick, Color::LightGreen, "Thick", "━─"),
    (BarStyle::Wave, Color::Yellow, "Wave", "≈˜"),
    (BarStyle::Pip, Color::LightRed, "Pip", "▪·"),
];

/// Renders symbol styles beside the options that benefit from horizontal space
pub(super) fn render_horizontal(frame: &mut Frame, tick: u64) {
    let columns = Layout::horizontal([29, 22]).spacing(2);
    let [styles, options] = columns.areas(frame.area());

    render_horizontal_styles(frame, styles, tick);
    render_horizontal_options(frame, options, tick);
}

/// Renders all symbol pairs as equal-width horizontal bars
fn render_horizontal_styles(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 17] = Layout::vertical([1; 17]).areas(area);
    let columns = Layout::horizontal([8, 7, 12]).spacing(1);
    let [style, symbols, preview] = columns.areas(rows[0]);
    frame.render_widget(span!(Modifier::BOLD; "Style"), style);
    frame.render_widget(span!(Modifier::BOLD; "Symbols"), symbols);
    frame.render_widget(span!(Modifier::BOLD; "Preview"), preview);

    for ((style, color, name, symbols), row) in zip(STYLES, &rows[1..]) {
        let [style_area, symbols_area, preview] = columns.areas(*row);
        let bar = BarSpinner::new(tick)
            .width(12)
            .arc_width(3)
            .bar_style(style)
            .arc_color(color);

        frame.render_widget(span!(color; name), style_area);
        frame.render_widget(span!(color; symbols), symbols_area);
        frame.render_widget(bar, preview);
    }
}

/// Splits horizontal behavior into motion, direction, and thickness comparisons
fn render_horizontal_options(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([5, 3, 7]).spacing(1);
    let [motions, directions, thicknesses] = rows.areas(area);

    render_horizontal_motions(frame, motions, tick);
    render_horizontal_directions(frame, directions, tick);
    render_horizontal_thicknesses(frame, thicknesses, tick);
}

/// Compares edge behavior with a block style that makes two-arc motions clear
fn render_horizontal_motions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 5] = Layout::vertical([1; 5]).areas(area);
    frame.render_widget(span!(Modifier::BOLD; "Motion"), rows[0]);
    let options = [
        ("Bounce", BarMotion::Bounce, Color::Cyan),
        ("Loop", BarMotion::Loop, Color::Magenta),
        ("Squeeze", BarMotion::Squeeze, Color::LightGreen),
        ("Radiate", BarMotion::Radiate, Color::Yellow),
    ];
    for ((name, motion, color), row) in zip(options, &rows[1..]) {
        let [label, preview] = Layout::horizontal([8, 13]).spacing(1).areas(*row);
        let bar = BarSpinner::new(tick)
            .width(13)
            .arc_width(3)
            .motion(motion)
            .bar_style(BarStyle::Block)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(bar, preview);
    }
}

/// Compares the two starting directions using the same bouncing bar
fn render_horizontal_directions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 3] = Layout::vertical([1; 3]).areas(area);
    frame.render_widget(span!(Modifier::BOLD; "Bounce direction"), rows[0]);
    let options = [
        ("Forwards", Spin::Clockwise),
        ("Backwards", Spin::CounterClockwise),
    ];
    for ((name, spin), row) in zip(options, &rows[1..]) {
        let [label, preview] = Layout::horizontal([9, 12]).spacing(1).areas(*row);
        let bar = BarSpinner::new(tick)
            .width(12)
            .arc_width(3)
            .spin(spin)
            .bar_style(BarStyle::Dot)
            .arc_color(Color::LightRed);
        frame.render_widget(span!(Color::LightRed; name), label);
        frame.render_widget(bar, preview);
    }
}

/// Shows cross-axis thickness with compact bars that fit on one row
fn render_horizontal_thicknesses(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 5]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Thickness"), heading);
    let columns = Layout::horizontal([4; 4]).spacing(1);
    let labels: [Rect; 4] = columns.areas(labels);
    let previews: [Rect; 4] = columns.areas(previews);
    let options = [
        (1, "t=1", Color::Cyan),
        (2, "t=2", Color::Magenta),
        (3, "t=3", Color::LightGreen),
        (4, "t=4", Color::Yellow),
    ];
    let options = zip(options, zip(labels, previews));
    for ((thickness, label, color), (label_area, preview)) in options {
        let bar = BarSpinner::new(tick)
            .width(4)
            .arc_width(2)
            .thickness(thickness)
            .bar_style(BarStyle::Block)
            .arc_color(color);
        frame.render_widget(span!(color; label), label_area);
        frame.render_widget(bar, preview);
    }
}

/// Renders symbol styles above the options that need taller animation tracks
pub(super) fn render_vertical(frame: &mut Frame, tick: u64) {
    let rows = Layout::vertical([17, 10]).spacing(1);
    let [styles_area, options_area] = rows.areas(frame.area());

    render_vertical_styles(frame, styles_area, tick);
    render_vertical_options(frame, options_area, tick);
}

/// Compares every symbol style in both vertical directions
///
/// Column widths follow the longer label in each row pair so the two rows align without padding
/// every item to the widest name in the gallery
fn render_vertical_styles(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([8, 8]).spacing(1);
    let rows: [Rect; 2] = rows.areas(area);
    let columns = Layout::horizontal([9, 8, 7, 7, 10, 8, 6, 7]);

    for (styles, row) in zip(STYLES.as_chunks::<8>().0, rows) {
        let tiles: [Rect; 8] = columns.areas(row);

        for ((style, color, name, symbols), tile) in zip(styles.iter().copied(), tiles) {
            let rows = Layout::vertical([1, 1, 1, 5]).areas(tile);
            let [heading, symbols_area, arrows, spinners] = rows;
            let tracks = Layout::horizontal([1, 1]).spacing(1);
            let [down_arrow, up_arrow] = tracks.areas(arrows);
            let [downwards_area, upwards_area] = tracks.areas(spinners);
            let downwards = BarSpinner::new(tick)
                .orientation(BarOrientation::Vertical)
                .width(1)
                .height(5)
                .arc_width(2)
                .spin(Spin::Clockwise)
                .bar_style(style)
                .arc_color(color);
            let upwards = BarSpinner::new(tick)
                .orientation(BarOrientation::Vertical)
                .width(1)
                .height(5)
                .arc_width(2)
                .spin(Spin::CounterClockwise)
                .bar_style(style)
                .arc_color(color);

            frame.render_widget(span!(color; name), heading);
            frame.render_widget(span!(color; symbols), symbols_area);
            frame.render_widget("↓", down_arrow);
            frame.render_widget("↑", up_arrow);
            frame.render_widget(downwards, downwards_area);
            frame.render_widget(upwards, upwards_area);
        }
    }
}

/// Places motion, direction, and thickness comparisons side by side
fn render_vertical_options(frame: &mut Frame, area: Rect, tick: u64) {
    let columns = Layout::horizontal([27, 17, 15]).spacing(2);
    let [motions, directions, thicknesses] = columns.areas(area);

    render_vertical_motions(frame, motions, tick);
    render_vertical_directions(frame, directions, tick);
    render_vertical_thicknesses(frame, thicknesses, tick);
}

/// Compares vertical motion modes on tall tracks so multi-arc motion remains legible
fn render_vertical_motions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 8]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Motion"), heading);
    let columns = Layout::horizontal([6, 4, 7, 7]).spacing(1);
    let labels: [Rect; 4] = columns.areas(labels);
    let previews: [Rect; 4] = columns.areas(previews);
    let motion_options = [
        ("Bounce", BarMotion::Bounce, Color::Cyan),
        ("Loop", BarMotion::Loop, Color::Magenta),
        ("Squeeze", BarMotion::Squeeze, Color::LightGreen),
        ("Radiate", BarMotion::Radiate, Color::Yellow),
    ];
    let motion_options = zip(motion_options, zip(labels, previews));
    for ((name, motion, color), (label, preview)) in motion_options {
        let bar = BarSpinner::new(tick)
            .orientation(BarOrientation::Vertical)
            .width(1)
            .height(8)
            .arc_width(2)
            .motion(motion)
            .bar_style(BarStyle::Block)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(bar, preview);
    }
}

/// Compares downward and upward starts using the same bouncing bar
fn render_vertical_directions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 8]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Bounce direction"), heading);
    let columns = Layout::horizontal([9, 7]).spacing(1);
    let labels: [Rect; 2] = columns.areas(labels);
    let previews: [Rect; 2] = columns.areas(previews);
    let direction_options = [
        ("Downwards", Spin::Clockwise),
        ("Upwards", Spin::CounterClockwise),
    ];
    let direction_options = zip(direction_options, zip(labels, previews));
    for ((name, spin), (label, preview)) in direction_options {
        let bar = BarSpinner::new(tick)
            .orientation(BarOrientation::Vertical)
            .width(1)
            .height(8)
            .arc_width(2)
            .spin(spin)
            .bar_style(BarStyle::Dot)
            .arc_color(Color::LightRed);
        frame.render_widget(span!(Color::LightRed; name), label);
        frame.render_widget(bar, preview);
    }
}

/// Shows vertical bars with increasing cross-axis thickness
fn render_vertical_thicknesses(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 8]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Thickness"), heading);
    let columns = Layout::horizontal([3; 4]).spacing(1);
    let labels: [Rect; 4] = columns.areas(labels);
    let previews: [Rect; 4] = columns.areas(previews);
    let thickness_options = [
        (1, "t=1", Color::Cyan),
        (2, "t=2", Color::Magenta),
        (3, "t=3", Color::LightGreen),
        (4, "t=4", Color::Yellow),
    ];
    let thickness_options = zip(thickness_options, zip(labels, previews));
    for ((thickness, name, color), (label, preview)) in thickness_options {
        let bar = BarSpinner::new(tick)
            .orientation(BarOrientation::Vertical)
            .height(8)
            .arc_width(2)
            .thickness(thickness)
            .bar_style(BarStyle::Block)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(bar, preview);
    }
}
