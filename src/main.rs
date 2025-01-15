mod account;
mod asset;

#[derive(Default)]
struct Portfolio {
    val: i32
}

#[derive(Debug, Clone, Copy)]
enum Message {
    INC,
    DEC
}

use iced::widget::{button, column, text, Column};

impl Portfolio {
    pub fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::INC),
            text(self.val).size(50),
            button("-").on_press(Message::DEC)
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::INC => self.val += 1,
            Message::DEC => self.val -= 1,
        }
    }
}

fn main() {
    iced::run("APP", Portfolio::update, Portfolio::view);
}
