use ui::Interface;

mod model;
mod ui;

fn main() {
	let ui: Interface = Interface::default();

	let res = ui.run();
	if res.is_err() {
		panic!("Interface error");
	}
}