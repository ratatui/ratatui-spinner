//! Flux preset and configuration gallery

use super::*;

/// Every built-in frame sequence with its label, glyphs, and gallery color
pub(super) const PRESETS: [(&[char], Color, &str, &str); 21] = [
    (FluxFrames::BRAILLE, Color::White, "Braille", "⣾⣷⣯⣟⡿⢿⣽⣻"),
    (FluxFrames::ORBIT, Color::Cyan, "Orbit", "⠁⠈⠐⠠⢀⡀⠄⠂"),
    (FluxFrames::CLASSIC, Color::Magenta, "Classic", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    (FluxFrames::LINE, Color::LightGreen, "Line", "│╱─╲"),
    (FluxFrames::BLOCK, Color::Yellow, "Block", "▖▘▝▗"),
    (FluxFrames::ARC, Color::LightRed, "Arc", "◜◝◞◟"),
    (FluxFrames::CLOCK, Color::LightCyan, "Clock", "◷◶◵◴"),
    (FluxFrames::MOON, Color::LightMagenta, "Moon", "◓◑◒◐"),
    (FluxFrames::TRIANGLES, Color::White, "Triangles", "▲▶▼◀"),
    (FluxFrames::PULSE, Color::Cyan, "Pulse", "⣀⣤⣶⣾⣿⣾⣶⣤"),
    (FluxFrames::BOUNCE, Color::Magenta, "Bounce", "⠉⠒⣀⠒"),
    (FluxFrames::HALF, Color::LightGreen, "Half", "▀▐▄▌"),
    (FluxFrames::SQUARE, Color::Yellow, "Square", "◰◳◲◱"),
    (FluxFrames::DICE, Color::LightRed, "Dice", "⚀⚁⚂⚃⚄⚅"),
    (FluxFrames::BAR, Color::LightCyan, "Bar", "▁▂▃▄▅▆▇█"),
    (FluxFrames::CORNERS, Color::LightMagenta, "Corners", "┌┐┘└"),
    (
        FluxFrames::CIRCLE_FILL,
        Color::White,
        "Circle fill",
        "○◔◑◕●",
    ),
    (FluxFrames::PISTON, Color::Cyan, "Piston", "▁▃▅▇█▇▅▃"),
    (FluxFrames::STAR, Color::Magenta, "Star", "✶✷✸✹"),
    (FluxFrames::PAIR, Color::LightGreen, "Pair", "⠉⠘⠰⢠⣀⡄⠆⠃"),
    (FluxFrames::DIAMOND, Color::Yellow, "Diamond", "◇◈◆◈"),
];

/// Renders the complete preset catalog followed by representative configuration changes
pub(super) fn render(frame: &mut Frame, tick: u64) {
    let rows = Layout::vertical([14, 11]).spacing(1);
    let [presets, options] = rows.areas(frame.area());

    render_presets(frame, presets, tick);
    render_options(frame, options, tick);
}

/// Renders presets in reading order as seven rows of three items
fn render_presets(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 7] = Layout::vertical([2; 7]).areas(area);
    let columns = Layout::horizontal([16; 3]).spacing(1);

    for (presets, row) in zip(PRESETS.chunks_exact(3), rows) {
        let tiles: [Rect; 3] = columns.areas(row);

        for ((frames, color, name, symbols), tile) in zip(presets.iter().copied(), tiles) {
            let [heading, preview] = Layout::vertical([1; 2]).areas(tile);
            let columns = Layout::horizontal([11, 4]).spacing(1);
            let [symbols_area, spinner_area] = columns.areas(preview);
            let spinner = FluxSpinner::new(tick).width(4).frames(frames).color(color);

            frame.render_widget(span!((color, Modifier::BOLD); name), heading);
            frame.render_widget(span!(color; symbols), symbols_area);
            frame.render_widget(spinner, spinner_area);
        }
    }
}

/// Groups behavioral options separately from size and custom-frame options
fn render_options(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([5, 5]).spacing(1);
    let [behavior, shape] = rows.areas(area);
    let [directions, phases] = Layout::horizontal([24; 2]).spacing(2).areas(behavior);
    let [sizes, custom] = Layout::horizontal([20, 28]).spacing(2).areas(shape);

    render_directions(frame, directions, tick);
    render_phases(frame, phases, tick);
    render_sizes(frame, sizes, tick);
    render_custom(frame, custom, tick);
}

/// Compares the same wave moving in both directions
fn render_directions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 3] = Layout::vertical([1; 3]).areas(area);
    frame.render_widget(span!(Modifier::BOLD; "Direction"), rows[0]);
    let options = [
        ("Clockwise", Spin::Clockwise, Color::Cyan),
        ("Counter-clockwise", Spin::CounterClockwise, Color::Magenta),
    ];
    for ((name, spin, color), row) in zip(options, &rows[1..]) {
        let [label, preview] = Layout::horizontal([17, 6]).spacing(1).areas(*row);
        let spinner = FluxSpinner::new(tick).width(6).spin(spin).color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}

/// Shows how adjacent-cell phase offsets change a wave
fn render_phases(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 5] = Layout::vertical([1; 5]).areas(area);
    frame.render_widget(span!(Modifier::BOLD; "Phase step"), rows[0]);
    let options = [
        ("step=0", 0, Color::LightGreen),
        ("step=1", 1, Color::Yellow),
        ("step=2", 2, Color::LightRed),
        ("step=4", 4, Color::LightCyan),
    ];
    for ((name, phase, color), row) in zip(options, &rows[1..]) {
        let [label, preview] = Layout::horizontal([7, 16]).spacing(1).areas(*row);
        let spinner = FluxSpinner::new(tick)
            .width(16)
            .phase_step(phase)
            .color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}

/// Shows the useful progression from one glyph to a two-dimensional field
fn render_sizes(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 3]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Size"), heading);
    let columns = Layout::horizontal([5, 6, 6]).spacing(1);
    let labels: [Rect; 3] = columns.areas(labels);
    let previews: [Rect; 3] = columns.areas(previews);
    let options = [
        ("1 × 1", 1, 1, Color::Cyan),
        ("6 × 1", 6, 1, Color::Magenta),
        ("6 × 3", 6, 3, Color::LightGreen),
    ];
    let options = zip(options, zip(labels, previews));
    for ((name, width, height, color), (label, preview)) in options {
        let spinner = FluxSpinner::new(tick)
            .width(width)
            .height(height)
            .color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}

/// Demonstrates that frame slices are not limited to the built-in presets
fn render_custom(frame: &mut Frame, area: Rect, tick: u64) {
    const FRAMES: &[char] = &['░', '▒', '▓', '█'];

    let rows = Layout::vertical([1, 1, 3]).areas(area);
    let [heading, symbols, preview] = rows;
    let spinner = FluxSpinner::new(tick)
        .width(12)
        .height(3)
        .frames(FRAMES)
        .color(Color::Yellow);

    frame.render_widget(span!(Modifier::BOLD; "Custom frames"), heading);
    frame.render_widget(span!(Color::Yellow; "░▒▓█"), symbols);
    frame.render_widget(spinner, preview);
}
