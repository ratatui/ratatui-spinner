//! Curated overview used as the README and crates.io hero image

use super::*;

/// Number of animation ticks for which the overview keeps one set of options visible
const OPTION_TICKS: u64 = 25;

/// Renders a compact sample of every spinner family
///
/// Fixed cell sizes fill the 890 × 460 hero recording without turning it into a terminal-sized GIF
pub(super) fn render(frame: &mut Frame, tick: u64) {
    let rows = Layout::vertical([1, 6, 1, 5]);
    let [title, top, _, bottom] = rows.areas(frame.area());
    frame.render_widget(
        span!((Color::White, Modifier::BOLD); "ratatui-spinner"),
        title,
    );

    let columns = Layout::horizontal([17, 17, 21]).spacing(2);
    let [linear, bar, flux] = columns.areas(top);
    render_linear(frame, linear, tick);
    render_bar(frame, bar, tick);
    render_flux(frame, flux, tick);

    let [circle, square, rect] = columns.areas(bottom);
    render_circle(frame, circle, tick);
    render_square(frame, square, tick);
    render_rect(frame, rect, tick);
}

/// Renders horizontal Linear samples with two narrow vertical companions
fn render_linear(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, body] = Layout::vertical([1, 5]).areas(area);
    frame.render_widget(span!((Color::Cyan, Modifier::BOLD); "Linear"), heading);

    let styles = match option(tick) {
        0 => [
            LinearStyle::Diamond,
            LinearStyle::Arrow,
            LinearStyle::Square,
            LinearStyle::Bar,
            LinearStyle::Braille,
        ],
        1 => [
            LinearStyle::Classic,
            LinearStyle::Square,
            LinearStyle::Braille,
            LinearStyle::Diamond,
            LinearStyle::Arrow,
        ],
        _ => [
            LinearStyle::Arrow,
            LinearStyle::Bar,
            LinearStyle::Classic,
            LinearStyle::Braille,
            LinearStyle::Square,
        ],
    };
    let [horizontal, forwards, backwards] = Layout::horizontal([13, 1, 1]).spacing(1).areas(body);
    let rows: [Rect; 5] = Layout::vertical([1; 5]).areas(horizontal);
    let flows = [
        Flow::Forwards,
        Flow::Backwards,
        Flow::Forwards,
        Flow::Backwards,
        Flow::Forwards,
    ];
    let colors = [
        Color::Cyan,
        Color::LightCyan,
        Color::White,
        Color::Cyan,
        Color::LightCyan,
    ];
    let options = zip(zip(styles, flows), colors);
    for (((style, flow), color), row) in zip(options, rows) {
        let spinner = LinearSpinner::new(tick)
            .total_slots(13)
            .lit_slots(3)
            .flow(flow)
            .linear_style(style)
            .active_color(color);
        frame.render_widget(spinner, row);
    }

    let vertical = LinearSpinner::new(tick)
        .direction(Direction::Vertical)
        .total_slots(5)
        .ticks_per_step(4)
        .linear_style(styles[0])
        .active_color(Color::Cyan);
    frame.render_widget(vertical, forwards);

    let vertical = LinearSpinner::new(tick)
        .direction(Direction::Vertical)
        .total_slots(5)
        .ticks_per_step(4)
        .flow(Flow::Backwards)
        .linear_style(styles[1])
        .active_color(Color::White);
    frame.render_widget(vertical, backwards);
}

/// Renders horizontal Bar samples with both vertical directions beside them
fn render_bar(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, body] = Layout::vertical([1, 5]).areas(area);
    frame.render_widget(span!((Color::Magenta, Modifier::BOLD); "Bar"), heading);

    let styles = match option(tick) {
        0 => [
            BarStyle::Braille,
            BarStyle::Block,
            BarStyle::Diamond,
            BarStyle::Dot,
            BarStyle::Progress,
        ],
        1 => [
            BarStyle::Shade,
            BarStyle::Square,
            BarStyle::Star,
            BarStyle::Heart,
            BarStyle::Thick,
        ],
        _ => [
            BarStyle::Arrow,
            BarStyle::Circle,
            BarStyle::Spark,
            BarStyle::Progress,
            BarStyle::Diamond,
        ],
    };
    let motions = [
        BarMotion::Bounce,
        BarMotion::Radiate,
        BarMotion::Loop,
        BarMotion::Squeeze,
        BarMotion::Bounce,
    ];
    let [horizontal, downwards, upwards] = Layout::horizontal([13, 1, 1]).spacing(1).areas(body);
    let rows: [Rect; 5] = Layout::vertical([1; 5]).areas(horizontal);
    let colors = [
        Color::Magenta,
        Color::LightMagenta,
        Color::White,
        Color::Magenta,
        Color::LightMagenta,
    ];
    let options = zip(zip(styles, motions), colors);
    for (((style, motion), color), row) in zip(options, rows) {
        let spinner = BarSpinner::new(tick)
            .width(13)
            .arc_width(3)
            .motion(motion)
            .bar_style(style)
            .arc_color(color);
        frame.render_widget(spinner, row);
    }

    let vertical = BarSpinner::new(tick)
        .orientation(BarOrientation::Vertical)
        .width(1)
        .height(5)
        .arc_width(2)
        .ticks_per_step(4)
        .bar_style(styles[1])
        .arc_color(Color::Magenta);
    frame.render_widget(vertical, downwards);

    let vertical = BarSpinner::new(tick)
        .orientation(BarOrientation::Vertical)
        .width(1)
        .height(5)
        .arc_width(2)
        .ticks_per_step(4)
        .spin(Spin::CounterClockwise)
        .bar_style(styles[2])
        .arc_color(Color::White);
    frame.render_widget(vertical, upwards);
}

/// Renders five Flux waves beside one single-cell spinner for every built-in preset
fn render_flux(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, body] = Layout::vertical([1, 5]).areas(area);
    frame.render_widget(span!((Color::LightGreen, Modifier::BOLD); "Flux"), heading);

    let [waves, preset_grid] = Layout::horizontal([6, 13]).spacing(2).areas(body);
    let waves: [Rect; 5] = Layout::vertical([1; 5]).areas(waves);
    let frames = match option(tick) {
        0 => [
            FluxFrames::ORBIT,
            FluxFrames::PULSE,
            FluxFrames::MOON,
            FluxFrames::BAR,
            FluxFrames::DIAMOND,
        ],
        1 => [
            FluxFrames::CLASSIC,
            FluxFrames::BAR,
            FluxFrames::DIAMOND,
            FluxFrames::ARC,
            FluxFrames::SQUARE,
        ],
        _ => [
            FluxFrames::ARC,
            FluxFrames::BOUNCE,
            FluxFrames::SQUARE,
            FluxFrames::TRIANGLES,
            FluxFrames::PAIR,
        ],
    };
    let colors = [
        Color::LightGreen,
        Color::Yellow,
        Color::LightGreen,
        Color::Cyan,
        Color::Yellow,
    ];
    for ((frames, color), area) in zip(zip(frames, colors), waves) {
        let spinner = FluxSpinner::new(tick)
            .width(6)
            .phase_step(2)
            .frames(frames)
            .color(color);
        frame.render_widget(spinner, area);
    }

    render_flux_presets(frame, preset_grid, tick);
}

/// Renders the complete preset catalog as a spaced seven-column grid
fn render_flux_presets(frame: &mut Frame, area: Rect, tick: u64) {
    let rows: [Rect; 3] = Layout::vertical([1; 3]).spacing(1).areas(area);
    for (presets, row) in zip(flux::PRESETS.chunks(7), rows) {
        let cells: [Rect; 7] = Layout::horizontal([1; 7]).spacing(1).areas(row);
        for ((frames, color, _, _), cell) in zip(presets.iter().copied(), cells) {
            let spinner = FluxSpinner::new(tick).frames(frames).color(color);
            frame.render_widget(spinner, cell);
        }
    }
}

/// Renders a Circle that changes radius when the overview changes options
fn render_circle(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, preview] = Layout::vertical([1, 4]).areas(area);
    frame.render_widget(span!((Color::Yellow, Modifier::BOLD); "Circle"), heading);
    let spinner = CircleSpinner::new(tick)
        .radius([6, 5, 4][option(tick)])
        .ticks_per_step(1)
        .arc_color(Color::Yellow);
    frame.render_widget(spinner, preview);
}

/// Renders a Square that changes size when the overview changes options
fn render_square(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, preview] = Layout::vertical([1, 4]).areas(area);
    frame.render_widget(span!((Color::LightRed, Modifier::BOLD); "Square"), heading);
    let spinner = SquareSpinner::new(tick)
        .size([4, 3, 2][option(tick)])
        .ticks_per_step(1)
        .arc_color(Color::LightRed);
    frame.render_widget(spinner, preview);
}

/// Renders a Rect that changes size and centre when the overview changes options
fn render_rect(frame: &mut Frame, area: Rect, tick: u64) {
    let [heading, preview] = Layout::vertical([1, 4]).areas(area);
    frame.render_widget(span!((Color::LightCyan, Modifier::BOLD); "Rect"), heading);
    let spinner = RectSpinner::new(tick)
        .shape(RectShape::Square([4, 3, 2][option(tick)]))
        .centre([Centre::Filled, Centre::Empty, Centre::Empty][option(tick)])
        .ticks_per_step(1)
        .outer_color(Color::LightCyan);
    frame.render_widget(spinner, preview);
}

/// Returns the current option set, cycling through three sets at a readable pace
fn option(tick: u64) -> usize {
    ((tick / OPTION_TICKS) % 3) as usize
}
