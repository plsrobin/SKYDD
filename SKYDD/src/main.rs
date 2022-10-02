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
use iced::{executor, Application, Command, Element, Settings, Text, Container, Length, Scrollable, Column};

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

}


async fn icedtest() -> iced::Result  {
    Hello::run(Settings::default())
}

//Shows text
//f
struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        match _message {
			Message::SyncRoom => {
				//do sutff
				
			}
            Message::ButtonPresed => {
                //do stuff
            }
		};

		Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new("bruh")
                .width(Length::Fill);
        let text2 = Text::new("bruh")
                .width(Length::Fill);
       let content = Column::new()
           .push(text)
           .push(text2);

       Container::new(
           content.width(Length::Fill),
        )
        .into()
    }
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
