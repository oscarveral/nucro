use iced::widget::{
    button, center, checkbox, column, horizontal_rule, pick_list, progress_bar,
    row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Center, Element, Fill, Subscription, Theme};

mod defaults;

#[derive(Default)]
pub struct Interface;

#[derive(Debug)]
enum Message {
	Placeholder
}

impl Interface {

	pub fn run(self) -> iced::Result{
		iced::application(defaults::WINDOW_TITLE, Interface::update, Interface::view).run()
	}

	fn update(&mut self, message: Message) {

	}
	fn view(&self) -> Element<Message> {
		let content = column![text('+')];
		center(content).into()
	}
}
