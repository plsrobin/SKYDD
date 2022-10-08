//BLock_on?? behöver för async i thread!?!?!?
use futures::executor::block_on;

//jag ÄLSKAR threads
use std::thread;

//Matrix någonting?
use std::{convert::TryFrom, str::SplitAsciiWhitespace};
use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent},
};

//iced (gui) imports
//use iced::widget::{container};
use iced::{executor, button, Button, Application, Command, Element, Settings, Text, Container, Length, Column};

//main
#[tokio::main]
async fn main() {


    //thread::spawn(|| icedtest())

    //matrixtest().await.map_err(|err| println!("{:?}", err)).ok();


    //STARTAR THREAD MEN ENDÅ INTE (TOKIO THREAD, funkar :) )
    tokio::spawn(async move{
        let _ = matrixtest().await;
    });

    //flytta till en annan porcess med tokio thread (eller inte)
    icedtest().await.map_err(|err| println!("{:?}", err)).ok();

}

//Messages between ui and other functions
#[derive(Debug, Clone)]
enum Message {
    SyncRoom,
    ButtonPresed,
    MsgFound(Result<matrixmsg, Error>),

}
//make error handling later
#[derive(Debug, Clone)]
enum Error {
}

#[derive(Debug, Clone)]
struct matrixmsg {
    msg: String,
}


async fn icedtest() -> iced::Result  {
    Hello::run(Settings::default())
}

//Shows text
enum Hello {
    Start,
    OnStart {
        btn: button::State,
    },
    LoadMsg,
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Message>) {
        (
            Hello::Start,
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        match _message {
			Message::SyncRoom => {
				*self = Hello::OnStart { 
                    btn: button::State::new(), 
                };
				
			}
            Message::ButtonPresed => {
                //do stuff
            }
            Message::MsgFound(Ok(matrixmsg)) => {
                // do stuff?
            }
            Message::MsgFound(Err(_error)) => {
                // do stuff? error handling
            }
		};

		Command::none()
    }

    fn view(&mut self) -> Element<Message> {

        //let button;

        let text3 = Text::new("LoadMsg");
        let text4 = Text::new("Start"); 

        let content = match self {
            Hello::Start => Column::new()
                .push(text4),
            Hello::LoadMsg => Column::new()
                .push(text3),
            Hello::OnStart { btn } => Column::new()
                .push(
                    button(btn, "habtegaber").on_press(Message::ButtonPresed),
                ),
        };

        Hello::OnStart { 
            btn,
            //https://github.com/iced-rs/iced/blob/0.4/examples/todos/src/main.register_event_handler
            //kolla TasteState::Editing nästa lektione :)
        let coolbutton = Column::new()
            .push(
                Button::new(
                btn,
                text4,
                )
            );
        let text = Text::new("bruh");
        let text2 = Text::new("bruh");
        let grafik = Column::new()
           .push(text)
           .push(text2)
           .push(content);
           //.push(testbtn);

       Container::new(grafik)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

//copy paste från pokemon exempel
fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        //.style(style::Button::Primary)
}

async fn matrixtest() -> anyhow::Result<()> {
	let userid = user_id!("@testuser3:norrland.xyz");
	let client = Client::builder().user_id(userid).build().await?;

    // First we need to log in.
    client.login(userid, "yahoogimmickchamberhypnoticechounfoundedbonedunpainted", None, None).await?;

	client.register_event_handler(|ev: SyncRoomMessageEvent| async move {
		println!("Received a message {:?}", ev);
        //funkar inte eftersom showmsg inte är global
        //showmsg = "{:?}";
	})
	.await;

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    client.sync(SyncSettings::default()).await;

    Ok(())	
}
