use super::*;

//imports

//iced (gui) imports
use iced::{executor, button, Button, Application, Command, Element, Text, Container, Length, Column};

//lets iced::command run async code
use async_trait::async_trait;


//Messages between ui and other functions
#[derive(Debug, Clone)]
pub enum Message {
    Search,
    /*TestMsg,*/
    MsgFound(Result<MatrixMsg, Error>),

}

//Gui states
pub enum Gui {
    Start {
        knapp_state: button::State,
    },
    Loading,
    LoadMsg {
        matrixmsg: MatrixMsg,
    },
}

#[async_trait]
impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Message>) {
        (
            Gui::Start { knapp_state: button::State::new() },
            //Command::perform(matrixmsg::search_msg(), Message::MsgFound), -- example for running
            //with command
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MsgFound(Ok(matrixmsg)) => {
                *self = Gui::LoadMsg {
                    matrixmsg,
                };

                Command::none()
            }
            Message::MsgFound(Err(_error)) => {
                println!("något error!?!?!?");

                Command::none()
            }
			Message::Search => match self {
                Gui::Loading => Command::none(),
                _ => {
                    *self = Gui::Loading;
                    Command::perform(MatrixMsg::search_msg(), Message::MsgFound)
                }
            },
		}
    }

    fn view(&mut self) -> Element<Message> {

        let text4 = Text::new("Start");

        let content = match self {
            Gui::Start { knapp_state, ..  } => Column::new()
                .push(text4)
                .push(button(knapp_state, "Hej :)").on_press(Message::Search)),
            Gui::LoadMsg { matrixmsg } => Column::new()
                .push(matrixmsg.view()),
            Gui::Loading => Column::new()
                .push(Text::new("Searching for messages...").size(40)),
        };

       Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

pub fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
}
