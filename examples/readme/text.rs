//! Text conversion examples embedded in table cells

use super::*;

/// Renders several conversion paths with their resulting live widgets
///
/// Fixed column widths fit the longest label in each column and define the tape's minimum width
pub(super) fn render(frame: &mut Frame, tick: u64) {
    let [title, table_area] = Layout::vertical([1, 12]).areas(frame.area());
    frame.render_widget(
        span!((Color::White, Modifier::BOLD); "Spinners as Text"),
        title,
    );

    let header = Row::new(["Task", "Spinner", "Conversion"]).style((Color::White, Modifier::BOLD));
    let rows = [
        Row::new([
            Cell::from("Downloading"),
            Cell::from(&FluxSpinner::new(tick).width(8).color(Color::Cyan)),
            Cell::from("Cell::from(&spinner)"),
        ]),
        Row::new([
            Cell::from("Indexing"),
            Cell::from(
                CircleSpinner::new(tick)
                    .radius(1)
                    .arc_color(Color::Magenta)
                    .to_text(),
            ),
            Cell::from("spinner.to_text()"),
        ])
        .height(3),
        Row::new([
            Cell::from("Compiling"),
            Cell::from(SquareSpinner::new(tick).size(2).arc_color(Color::Yellow)),
            Cell::from("Into<Text>"),
        ])
        .height(3),
        Row::new([
            Cell::from("Uploading"),
            Cell::from(
                LinearSpinner::new(tick)
                    .total_slots(8)
                    .active_color(Color::LightGreen)
                    .to_lines(),
            ),
            Cell::from("spinner.to_lines()"),
        ]),
        bar_row(tick),
    ];
    let widths = [
        Constraint::Length(12),
        Constraint::Length(14),
        Constraint::Length(27),
    ];
    let table = Table::new(rows, widths).header(header).column_spacing(1);
    frame.render_widget(table, table_area);
}

/// Builds the one row that must supply dimensions before converting a Bar
///
/// Bar has no intrinsic width, so this row also demonstrates combining its lines with a label
fn bar_row(tick: u64) -> Row<'static> {
    let bar = BarSpinner::new(tick).arc_color(Color::LightRed);
    let mut lines = vec![Line::from("step 3/5")];
    lines.extend(bar.to_lines(12, 1));
    Row::new([
        Cell::from("Building"),
        Cell::from(lines),
        Cell::from("bar.to_lines(12, 1)"),
    ])
    .height(2)
}
