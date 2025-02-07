mod model;
mod ui;

use crossterm::event::{self, Event};
use ratatui::{widgets::Block, DefaultTerminal, Frame};
use std::io::Result;

fn main() {
	let terminal = ratatui::init();
	let res = run(terminal);
	ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		terminal.draw(render)?;
		if matches!(event::read()?, Event::Key(_)) {
			break Ok(());
		}
	}
}

fn render(frame: &mut Frame) {
	frame.render_widget(Block::bordered().title("Nucro"), frame.area());
}
