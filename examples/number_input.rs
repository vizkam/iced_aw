// This example demonstrates how to use the number input widget
//
// It was written by leang27 <52003343+leang27@users.noreply.github.com>

use iced::{
    widget::{Container, Row, Text},
    Alignment, Element, Length,
};
use iced_aw::number_input;

#[derive(Default, Debug)]
pub struct NumberInputDemo {
    value: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumInpChanged(u8),
    NumInpSubmitted,
}

fn main() -> iced::Result {
    iced::application(
        NumberInputDemo::new,
        NumberInputDemo::update,
        NumberInputDemo::view,
    )
    .title("Number input example")
    .window_size(iced::Size {
        width: 250.0,
        height: 200.0,
    })
    .run()
}

impl NumberInputDemo {
    fn new() -> NumberInputDemo {
        NumberInputDemo {
            value: u8::default(),
        }
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::NumInpChanged(val) => {
                println!("Value changed to {:?}", val);
                self.value = val;
            }
            Message::NumInpSubmitted => {
                println!("Value submitted");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let lb_minute = Text::new("Number Input:");
        let txt_minute = number_input(&self.value, 0..=10, Message::NumInpChanged)
            .style(number_input::number_input::primary)
            .on_submit(Message::NumInpSubmitted)
            .step(1);

        Container::new(
            Row::new()
                .spacing(10)
                .align_y(Alignment::Center)
                .push(lb_minute)
                .push(txt_minute),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}
