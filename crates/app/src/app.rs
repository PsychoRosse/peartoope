use iced::Theme;
use iced::widget::{Column, button, column, text};

#[derive(Default)]
struct Counter {
    value: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            text(format!("Счётчик: {}", self.value)).size(28),
            button("+1").on_press(Message::Increment),
            button("-1").on_press(Message::Decrement),
        ]
        .spacing(12)
        .padding(24)
    }
}

pub fn go() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .theme(Theme::Dark)
        .centered()
        .run()
}
