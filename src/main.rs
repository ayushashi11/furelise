pub mod style;
use iced::{button::State, Align, Button, Column, Element, Row, Sandbox, Settings, Text};
struct Counter<'a> {
    value: Column<'a, Message>,
    inc: State,
    dec: State,
}
#[derive(Copy, Clone, Debug)]
enum Message {
    Nop,
    Inc,
    Dec,
}

impl<'a> Default for Counter<'a>{
    fn default() -> Self{
        Self{value: Column::new().push(Text::new("Hello world!")).into(), inc: State::default(), dec: State::default()}
    }
}
impl Default for Message{
    fn default() -> Self{
        Message::Nop
    }
}
impl<'a> Sandbox for Counter<'a> {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }
    fn title(&self) -> String {
        String::from("Counter")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Inc => {
                
            }
            Message::Dec => {
            }
            _ => {}
        }
    }
    fn view(&mut self) -> Element<Message> {
        Row::new()
            .align_items(Align::End)
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.inc, Text::new("Inc"))
                            .on_press(Message::Inc)
                            .style(style::Theme::Dark),
                    )
                    //.push(self.value)
                    .push(
                        Button::new(&mut self.dec, Text::new("Dec"))
                            .on_press(Message::Dec)
                            .style(style::Theme::Dark),
                    ),
            )
            .into()
    }
}
pub fn main() -> iced::Result {
    let settings = Settings::default();
    let st = "hello".to_string();
    println!("{:#?}", st);
    Counter::run(settings)
}
