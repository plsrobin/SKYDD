use super::*;

//imports

//iced (gui) imports
use iced::{executor, Application, Command, Element, Length};
use iced::{button, Button, Column, Row, Text, Container}; 
//use iced_native::widget::{button};

use async_trait::async_trait;

use iced_native::keyboard::KeyCode;
use iced_native::subscription::{self, Subscription};

//use iced_native::widget::Column;
use matrix_sdk::ruma::api::client::push;
/*
#[derive(Debug, Clone)]
pub struct Events {
    last: Vec<iced_native::Event>,
}*/

//Messages between ui and other functions
#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    EventOccurred(iced_native::Event),
    Search,
    MsgFound(Result<MatrixMsg, Error>),
}

//Gui states
pub enum Gui {
    MainView {
        msg: MatrixMsg,
        room: MatrixMsg,
    },
    Events {
        last: iced_native::Event,
    },
    Start {
        knapp_state: iced_native::widget::button::State,
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
            Message::KeyReleased(keycode) => {
                Command::none()
            }
            Message::KeyPressed(keycode) => {
                match keycode {
                    KeyCode::A => {
                        println!("A was pressed!");
                        *self = Gui::Loading;
                        Command::perform(MatrixMsg::search_msg(), Message::MsgFound)
                    },
                    _ => {
                        println!("something was pressed!");
                        Command::none()
                    },
                }
            }
            Message::EventOccurred(last) => {
                    *self = Gui::Events { 
                        last,
                    };
                    
                    Command::none()
                }
            Message::MsgFound(Ok(matrixmsg)) => {
               /* *self = Gui::LoadMsg {
                    matrixmsg,
                }; */
                *self = Gui::MainView {
                    msg: matrixmsg.clone(),
                    room: matrixmsg,
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

    fn subscription(&self) -> Subscription<Message> {
        //iced_native::subscription::events().map(Message::EventOccurred)
        use iced_native::event::Event;
        use iced_native::keyboard;

        subscription::events_with(|event, status| {
            match event {
                Event::Keyboard(e) => {
                    match e {
                        keyboard::Event::KeyPressed{key_code, modifiers: _} => {
                            Some(Message::KeyPressed(key_code))
                        },
                        keyboard::Event::KeyReleased{key_code, modifiers: _} => {
                            Some(Message::KeyReleased(key_code))
                        }
                        _ => None
                    }        
                },
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {

        let text_start= Text::new("Start");
        /*
        //msg 1
        let msg_icon1 = Column::new()
            .push(Text::new("\"Bild1\""));
        let msg_text1 = Column::new()
            .push(Text::new("User1"))
            .push(Text::new("Medelande1"));
        let example_msg1 = Row::new()
            .push(msg_icon1)
            .padding(10)
            .push(msg_text1);

        //msg 2
        let msg_icon2 = Column::new()
            .push(Text::new("\"Bild2\""));
        let msg_text2 = Column::new()
            .push(Text::new("User2"))
            .push(Text::new("Medelande2"));
        let example_msg2 = Row::new()
            .push(msg_icon2)
            .padding(10)
            .push(msg_text2);

        //msg 3
        let msg_icon3 = Column::new()
            .push(Text::new("\"Bild3\""));
        let msg_text3 = Column::new()
            .push(Text::new("User3"))
            .push(Text::new("Medelande3"));
        let example_msg3 = Row::new()
            .push(msg_icon3)
            .padding(10)
            .push(msg_text3);
        
        //msg 4
        let msg_icon4 = Column::new()
            .push(Text::new("\"Bild4\""));
        let msg_text4 = Column::new()
            .push(Text::new("User4"))
            .push(Text::new("längtar till den obligatoriska diskussionen vid jul när man tittar på kalle anka och gubbarna snackar om hur dom har tagit bort klippet med svarta flickan i tomtens verkstad och hur det är trams osv osv"));
        let example_msg4 = Row::new()
            .push(msg_icon4)
            .padding(10)
            .push(msg_text4);

        //msg 5
        let msg_icon5 = Column::new()
            .push(Text::new("\"Bild5\""));
        let msg_text5 = Column::new()
            .push(Text::new("User5"))
            .push(Text::new("Medelande5"));
        let example_msg5 = Row::new()
            .push(msg_icon5)
            .padding(10)
            .push(msg_text5);
        
        //current messages
        let message_view = Column::new()
           /* .push(example_msg1)
            .push(example_msg2)
            .push(example_msg3)
            .push(example_msg4) */
            .push(example_msg5) 
            //.push(matrixmsg.view())
            .width(Length::Fill)
            .height(Length::FillPortion(8))
            .align_items(iced::Alignment::Start);
        //Room description
        let current_desc = Column::new()
            .push(Text::new("Crab (jeff@norrland.xyz)"));
        //current room's icon (description)
        let current_icon = Column::new()
            .push(Text::new("\"Bild\""));
        //description of rooms
        let room_desc = Row::new()
            .push(current_icon)
            .push(current_desc)
            .height(Length::FillPortion(1))
            .width(Length::Fill);
        let middle_view = Column::new()
            .push(room_desc)
            .push(message_view)
            .width(Length::FillPortion(5))
            .height(Length::Fill);

        //list of rooms
        /*let room_view = Column::new()
            .push(Text::new("Rum1"))
            .push(Text::new("Rum2"))
            .push(Text::new("Rum3"))
            .push(Text::new("Rum4"))
            .push(Text::new("Rum4"))
            .push(Text::new("Rum4"))
            .push(Text::new("Rum4"))
            .push(Text::new("Rum4"))
            .push(Text::new("Rum5"))
            .push(Text::new("Rum6"))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_items(iced::Alignment::Start); */
        let input = Column::new()
            //gö senare?
            //.push(TextInput::new("...", value, Message::CommandChange)
            .push(Text::new("TextInput"))
            .height(Length::FillPortion(2))
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);
        let top = Row::new()
            .push(room_view)
            .push(middle_view)
            .height(Length::FillPortion(10));
        let overview = Column::new()
            .push(top)
            .push(input); */
        let content = match self {
            Gui::MainView { msg, room } => Column::new()
                .push(Row::new()
                      .push(Column::new()
                        //joined rooms view
                            .push(Column::new()
                                .push(room.rms())
                                )
                            //style
                            .width(Length::FillPortion(2))
                            )
                      //current room  view
                      .push(Column::new()
                            //Room Description
                            .push(Row::new()
                                  //icon
                                  .push(Column::new()
                                        .push(Text::new("\"Bild\""))
                                        )
                                  //room info
                                  .push(Column::new()
                                        .push(Text::new("Crab (jeff@norrland.xyz)"))
                                        .push(Text::new("This is the current room description"))
                                        .height(Length::FillPortion(2))
                                        )
                                  //style desc
                                  .height(Length::FillPortion(2))
                                  )
                            //Messages
                            .push(Column::new()
                                .push(msg.view())
                                //style msg
                                .height(Length::FillPortion(5))
                                )
                            .push(Column::new()
                                .push(Text::new("TextInput"))
                                .height(Length::FillPortion(2))
                                .align_items(iced::Alignment::Start)
                        )
                            //style desc & msg & input
                            .width(Length::FillPortion(8))
                        ) 
                        //style everything
                    )
                    
                //inputfield
                 .push(Column::new()
                        .push(Text::new("TextInput"))
                        .height(Length::FillPortion(2))
                        .align_items(iced::Alignment::Start)
                        )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(iced::Alignment::Start),
                //.push(example_msg1)
                //.push(example_msg2)
                //.push(example_msg3)
                //.push(example_msg4)
                //.push(example_msg5),
            Gui::Start { knapp_state, ..  } => Column::new()
                .push(text_start)
                .push(button(knapp_state, "Hej :)").on_press(Message::Search)),
            Gui::LoadMsg { matrixmsg } => Column::new()
                .push(matrixmsg.view()),
            Gui::Loading => Column::new()
                .push(Text::new("Searching for messages...").size(40)),
            Gui::Events { last, .. } => Column::new()
                .push(Text::new("rr")),
        };

       Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        //.center_x()
        //.center_y()
        .into()
    }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
}
