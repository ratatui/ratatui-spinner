//! Circle size, arc-length, and direction gallery

use super::*;

/// Renders the three independent dimensions of Circle configuration
pub(super) fn render(frame: &mut Frame, tick: u64) {
    let rows = Layout::vertical([13, 5, 5]).spacing(1);
    let [radii, arcs, directions] = rows.areas(frame.area());

    render_radii(frame, radii, tick);
    render_arcs(frame, arcs, tick);
    render_directions(frame, directions, tick);
}

/// Compares representative radii while allocating taller cells to larger circles
fn render_radii(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 4, 6]).spacing(1);
    let [heading, small, large] = rows.areas(area);
    frame.render_widget(span!(Modifier::BOLD; "Radius"), heading);
    let columns = Layout::horizontal([8, 9, 11]).spacing(1);
    let radii = [
        ("r=2", 2, Color::Cyan),
        ("r=3", 3, Color::Magenta),
        ("r=4", 4, Color::LightGreen),
        ("r=5", 5, Color::Yellow),
        ("r=6", 6, Color::LightRed),
        ("r=8", 8, Color::LightCyan),
    ];
    let rows = [(small, 3), (large, 5)];
    for (radii, (row, height)) in zip(radii.as_chunks::<3>().0, rows) {
        let tiles: [Rect; 3] = columns.areas(row);

        for ((name, radius, color), tile) in zip(radii.iter().copied(), tiles) {
            let [label, preview] = Layout::vertical([1, height]).areas(tile);
            let spinner = CircleSpinner::new(tick)
                .radius(radius)
                .ticks_per_step(3)
                .arc_color(color);
            frame.render_widget(span!(color; name), label);
            frame.render_widget(spinner, preview);
        }
    }
}

/// Compares automatic and explicit bright-arc lengths at a fixed radius
fn render_arcs(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 3]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Arc length"), heading);
    let columns = Layout::horizontal([8; 4]).spacing(1);
    let labels: [Rect; 4] = columns.areas(labels);
    let previews: [Rect; 4] = columns.areas(previews);
    let options = [
        ("auto", 0, Color::Cyan),
        ("len=2", 2, Color::Magenta),
        ("len=6", 6, Color::LightGreen),
        ("len=12", 12, Color::Yellow),
    ];
    let options = zip(options, zip(labels, previews));
    for ((name, arc_len, color), (label, preview)) in options {
        let spinner = CircleSpinner::new(tick)
            .radius(5)
            .arc_len(arc_len)
            .ticks_per_step(3)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}

/// Compares clockwise and counter-clockwise motion at a fixed size
fn render_directions(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 3]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Direction"), heading);
    let columns = Layout::horizontal([9, 17]).spacing(2);
    let labels: [Rect; 2] = columns.areas(labels);
    let previews: [Rect; 2] = columns.areas(previews);
    let options = [
        ("Clockwise", Spin::Clockwise, Color::LightRed),
        (
            "Counter-clockwise",
            Spin::CounterClockwise,
            Color::LightCyan,
        ),
    ];
    let options = zip(options, zip(labels, previews));
    for ((name, spin, color), (label, preview)) in options {
        let spinner = CircleSpinner::new(tick)
            .radius(5)
            .spin(spin)
            .ticks_per_step(3)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}
