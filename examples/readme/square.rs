//! Square size, centre, and direction gallery

use super::*;

/// Renders size examples before the centre and direction matrix
pub(super) fn render(frame: &mut Frame, tick: u64) {
    let rows = Layout::vertical([7, 10]).spacing(1);
    let [sizes, options] = rows.areas(frame.area());

    render_sizes(frame, sizes, tick);
    render_options(frame, options, tick);
}

/// Compares representative sizes and slows larger perimeters to a similar visual pace
fn render_sizes(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 1, 5]).areas(area);
    let [heading, labels, previews] = rows;
    frame.render_widget(span!(Modifier::BOLD; "Size"), heading);
    let columns = Layout::horizontal([6, 9, 11]).spacing(1);
    let labels: [Rect; 3] = columns.areas(labels);
    let previews: [Rect; 3] = columns.areas(previews);
    let options = [
        ("size=2", 2, 4, Color::Cyan),
        ("size=3", 3, 5, Color::Magenta),
        ("size=4", 4, 6, Color::LightGreen),
    ];
    let options = zip(options, zip(labels, previews));
    for ((name, size, ticks_per_step, color), (label, preview)) in options {
        let spinner = SquareSpinner::new(tick)
            .size(size)
            .ticks_per_step(ticks_per_step)
            .arc_color(color);
        frame.render_widget(span!(color; name), label);
        frame.render_widget(spinner, preview);
    }
}

/// Crosses both centre treatments with both spin directions
fn render_options(frame: &mut Frame, area: Rect, tick: u64) {
    let rows = Layout::vertical([1, 4, 4]).spacing(1);
    let [headings, filled, empty] = rows.areas(area);
    let columns = Layout::horizontal([7, 7, 7]).spacing(2);
    let [centre, clockwise, counter_clockwise] = columns.areas(headings);
    frame.render_widget(span!(Modifier::BOLD; "Centre"), centre);
    frame.render_widget(span!(Modifier::BOLD; "↻"), clockwise);
    frame.render_widget(span!(Modifier::BOLD; "↺"), counter_clockwise);

    let options = [
        (filled, "Filled", Centre::Filled, Color::Yellow),
        (empty, "Empty", Centre::Empty, Color::LightRed),
    ];
    for (row, name, centre, color) in options {
        let [label, clockwise, counter_clockwise] = columns.areas(row);
        let clockwise_spinner = SquareSpinner::new(tick)
            .size(3)
            .centre(centre)
            .ticks_per_step(5)
            .arc_color(color)
            .spin(Spin::Clockwise);
        let counter_clockwise_spinner = SquareSpinner::new(tick)
            .size(3)
            .centre(centre)
            .ticks_per_step(5)
            .arc_color(color)
            .spin(Spin::CounterClockwise);

        frame.render_widget(span!(color; name), label);
        frame.render_widget(clockwise_spinner, clockwise);
        frame.render_widget(counter_clockwise_spinner, counter_clockwise);
    }
}
