pub mod style;
use iced::{
    button::State, Align, Button, Column, Element, Sandbox, Settings, Text, Row
};
#[derive(Default)]
struct Counter{
    value: String,
    inc: State,
    dec: State
}
#[derive(Copy, Clone, Debug)]
enum Message{
    Inc,
    Dec
}
impl Sandbox for Counter{
    type Message = Message;
    fn new() -> Self{
        Self::default()
    }
    fn title(&self) -> String{
        String::from("Counter")
    }
    fn update(&mut self, message: Self::Message){
        match message{
            Message::Inc => {
                self.value.push('a');
            },
            Message::Dec => {
                self.value.pop();
            }
        }
    }
    fn view(&mut self) -> Element<Message>{
        Row::new()
            .align_items(Align::End)
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.inc, Text::new("Increment")).on_press(Message::Inc).style(style::Theme::Dark)
                    )
                    .push(Text::new(self.value.to_string()).size(50))
                    .push(Button::new(&mut self.dec, Text::new("Decrement")).on_press(Message::Dec).style(style::Theme::Dark))
            ).into()
    }
}
pub fn main() -> iced::Result{
    let settings = Settings::default();
    Counter::run(settings)
}
