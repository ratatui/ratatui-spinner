//! Compact scenes used by the README recordings
//!
//! Run one scene with `cargo run --example readme -- <scene>` and pass
//! `--help` to print the available scene names
//!
//! The scene modules own their layouts and representative configurations. This module keeps the
//! shared terminal lifecycle and tick cadence in one place so every VHS recording advances in the
//! same way

mod bar;
mod circle;
mod flux;
mod linear;
mod overview;
mod rect;
mod square;
mod text;

use std::iter::zip;
use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier};
use ratatui::text::Line;
use ratatui::widgets::{Cell, Row, Table};
use ratatui::{DefaultTerminal, Frame};
use ratatui_macros::span;
use ratatui_spinner::{
    BarMotion, BarOrientation, BarSpinner, BarStyle, Centre, CircleSpinner, Direction, Flow,
    FluxFrames, FluxSpinner, LinearSpinner, LinearStyle, RectShape, RectSpinner, Spin,
    SquareSpinner,
};

/// Time between animation ticks
///
/// The 80 ms cadence keeps detailed motion readable and aligns with both 25 and 50 fps recordings
const FRAME_TIME: Duration = Duration::from_millis(80);

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut app = App {
        scene: cli.scene,
        tick: 0,
    };
    ratatui::run(|terminal| app.run(terminal))
}

/// Selects one README scene from the command line
#[derive(Parser)]
struct Cli {
    /// Spinner scene to render
    #[arg(value_enum)]
    scene: Scene,
}

/// A separately recordable README image
#[derive(Clone, Copy, ValueEnum)]
enum Scene {
    Overview,
    LinearHorizontal,
    LinearVertical,
    BarHorizontal,
    BarVertical,
    Flux,
    Circle,
    Square,
    Rect,
    Text,
}

/// Animation state shared by every scene
struct App {
    scene: Scene,
    tick: u64,
}

impl App {
    /// Draws and advances the selected scene until the user quits
    ///
    /// Polling keeps the loop from busy-waiting and accounts for the time spent rendering
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut next_frame = Instant::now();
        loop {
            terminal.draw(|frame| self.render(frame))?;

            next_frame += FRAME_TIME;
            let timeout = next_frame.saturating_duration_since(Instant::now());
            if event::poll(timeout)? {
                let event = event::read()?;
                if let Some(key) = event.as_key_press_event() {
                    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        break Ok(());
                    }
                }
            }

            self.tick = self.tick.wrapping_add(1);
        }
    }

    /// Routes a frame to the selected scene without coupling scene modules to CLI state
    fn render(&self, frame: &mut Frame) {
        match self.scene {
            Scene::Overview => overview::render(frame, self.tick),
            Scene::LinearHorizontal => linear::render_horizontal(frame, self.tick),
            Scene::LinearVertical => linear::render_vertical(frame, self.tick),
            Scene::BarHorizontal => bar::render_horizontal(frame, self.tick),
            Scene::BarVertical => bar::render_vertical(frame, self.tick),
            Scene::Flux => flux::render(frame, self.tick),
            Scene::Circle => circle::render(frame, self.tick),
            Scene::Square => square::render(frame, self.tick),
            Scene::Rect => rect::render(frame, self.tick),
            Scene::Text => text::render(frame, self.tick),
        }
    }
}
