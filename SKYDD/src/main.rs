//BLock_on?? behöver för async i thread!?!?!?
use futures::{executor::block_on, TryFutureExt};
//test
use tokio::time::{sleep, Duration};
//test
//Matrix någonting?
use std::{convert::TryFrom, str::SplitAsciiWhitespace, };
use matrix_sdk::{
    Client, config::SyncSettings, room::MessagesOptions, 
    ruma::{room_id, server_name, user_id,  events::{room::message::{SyncRoomMessageEvent, OriginalSyncRoomMessageEvent, RoomMessageEventContent, TextMessageEventContent, MessageType,}, }, api::client::room::get_room_event}, ClientBuildError, ClientBuilder, ruma::{events::room::member::StrippedRoomMemberEvent,}, room::Room,
    ruma::events::{
        AnyMessageLikeEvent, AnyMessageLikeEventContent, AnyStateEvent, AnyToDeviceEvent,
    }, HttpError,
};

//iced (gui) imports
//use iced::widget::{container};
use iced::{executor, button, Button, Application, Command, Element, Settings, Text, Container, Length, Column, pane_grid::Direction};

use url::{Url, ParseError};

//command kan köra asynckod
use async_trait::async_trait;

//main
#[tokio::main]
async fn main() -> iced::Result {
    Hello::run(Settings::default())

    //thread::spawn(|| icedtest())

    //matrixtest().await.map_err(|err| println!("{:?}", err)).ok();


    //STARTAR THREAD MEN ENDÅ INTE (TOKIO THREAD, funkar :) )
 /*   tokio::spawn(async move{
        let _ = matrixtest().await;
    });*/

    //icedtest().await.map_err(|err| println!("{:?}", err)).ok();

}

/*async fn testmsgfunc() -> () {
    Message::TestMsg;
    println!("EXECIUTETED!!");
    //Command::perform(future, f)
} */

//Messages between ui and other functions
#[derive(Debug, Clone)]
enum Message {
    Search,
    /*TestMsg,*/
    MsgFound(Result<matrixmsg, Error>),

}
//make error handling later
#[derive(Debug, Clone)]
enum Error {
    ClientBuildError,
    MatrixError,
    ParseError,
    HttpError,
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        dbg!(error);
        Error::ParseError
    }
}

impl From<matrix_sdk::HttpError> for Error {
    fn from(error: HttpError) -> Error {
        dbg!(error);
        Error::HttpError
    }
}

impl From<matrix_sdk::ClientBuildError> for Error {
    fn from(error: ClientBuildError) -> Error {
        dbg!(error);
        Error::ClientBuildError
    }

}
impl From<matrix_sdk::Error> for Error {
    fn from(error: matrix_sdk::Error) -> Error {
        dbg!(error);
        Error::MatrixError
    }
}

#[derive(Debug, Clone)]
struct matrixmsg {
    msg: String,
}
impl matrixmsg {
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Text::new(&self.msg)
                    .size(30)
                    .width(Length::Fill),
                )
                .into()
    }
    async fn search_msg() -> anyhow::Result<matrixmsg, Error> {
        //let response = matrixtest().await.unwrap().to_string();
        //println!("sent string!");
	    let userid = user_id!("@testuser3:norrland.xyz");
	    let client = Client::builder().user_id(userid).build().await?;
        client.login(userid, "yahoogimmickchamberhypnoticechounfoundedbonedunpainted", None, None).await?;
        //let room = client.get_room(room_id!("!FVZaPevCZhhurovOAA:norrland.xyz"));
        //let options = MessagesOptions::backward("t47429-4392820_219380_26003_2265");
        //let response = room.messages(options).await;
        //client.sync(SyncSettings::default()).await;
        //DETTA FUNGERAR INTE ÄN, HUR FAN GÖR JAG MESSAGE EVENTS TILL STRING?
       // OVAN ÄR SKIT (kanske)
       
        //let mut client_builder = Client::builder().homeserver_url("norrland.xyz");

        //crypto cache?? Den sparar kryptionsnycklar lokalt tror inget annat?
        //let home = dirs::data_dir().expect("no home directory found").join("getting_started");
        //client_builder = client_builder.sled_store(home, None).await?;

       	//woah vi har inte crashat än... eller?
       	//println!("1. logged in as {userid}, initial sync is next..."); 
		
		//initial sync, uuuhhh 
        println!("starting sync_once");
		client.sync_once(SyncSettings::default()).await.unwrap();
        //println!("2. initial sync done!, addimg message event..."); 
		
	    //vet inte vad detta innebär. Förklaringen var: "since we called `sync_once` before we
        //entered our sync loop we must pass \n that sync token to `sync`	
		//let settings = SyncSettings::default().token(client.sync_token().await.unwrap());

        //detta fortsätter synca för evigt, det vill inte JAG
        //client.sync(settings).await?;
        //println!("1. started proccess!!!");
        
        //let homeserver_url = Url::parse("https:://matrix.norrland.xyz")?; 
        //let hms_url = homeserver("norrland.xyz")
        //println!("2. Defined homerserver, defining client next");
        //let client = Client::new("norrland.xyz").await?;
        //println!("3. Defined Client, defining user next");
        //let user = "testuser3";
        //println!("4. Defined user, defining response next");

        /*let response = client
            .login_username(user, "yahoogimmickchamberhypnoticechounfoundedbonedunpainted")
            .initial_device_display_name("uuuuhhh")
            .send()
            .await?; */
        println!("last step...");
        //denna fungerar bra som fan 
        //let respond = client.access_token().unwrap();
        
        //detta var istället jävligt jobbigt
        let room = client.get_room(room_id!("!FVZaPevCZhhurovOAA:norrland.xyz")).unwrap();
        let options = MessagesOptions::new(matrix_sdk::ruma::api::client::message::get_message_events::v3::Direction::Forward);
        let message = room.messages(options).await.unwrap().chunk.last().unwrap().event.json().to_string();
        Ok(matrixmsg {
            msg: message,
        })
    }
}

//Shows text
enum Hello {
    Start {
        knapp_state: button::State,
    },
    Loading,
    LoadMsg {
        matrixmsg: matrixmsg,
    },
}
#[async_trait]
impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Message>) {
        (
            Hello::Start { knapp_state: button::State::new() },
            //Command::perform(matrixmsg::search_msg(), Message::MsgFound),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MsgFound(Ok(matrixmsg)) => {
                *self = Hello::LoadMsg {
                    matrixmsg, 
                };

                Command::none()
            }
            Message::MsgFound(Err(_error)) => {
                println!("något error!?!?!?");

                Command::none()
            }
			Message::Search => match self { 
                Hello::Loading => Command::none(),
                _ => {
                    *self = Hello::Loading;
                    Command::perform(matrixmsg::search_msg(), Message::MsgFound)
                }
            },
		}
    }

    fn view(&mut self) -> Element<Message> {

        //let button;

        //fuckyou text3
        //let text3 = Text::new("LoadMsg");
        let text4 = Text::new("Start"); 
        //*self = Hello::LoadMsg; - funkade
        //testmsgfunc(); - funkade inte
        //Message::TestMsg; - funkade inte

        let content = match self {
            Hello::Start { knapp_state, ..  } => Column::new()
                .push(text4)
                .push(button(knapp_state, "Hej :)").on_press(Message::Search)),
            Hello::LoadMsg { matrixmsg } => Column::new()
                .push(matrixmsg.view()),
            Hello::Loading => Column::new()
                .push(Text::new("Searching for messages...").size(40)),
        };

        let text = Text::new("bruh");
        let text2 = Text::new("bruh");
        
        //let mut state = button::State::new();
        //let knapp = Button::new(&mut state, Text::new("hej"));

        /*let grafik = Column::new()
           .push(text)
           .push(text2);
           //.push(testbtn); */

       Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
}

async fn matrixtest() -> anyhow::Result<String> {
	let userid = user_id!("@testuser3:norrland.xyz");
	let client = Client::builder().user_id(userid).build().await?;

    // First we need to log in.
    client.login(userid, "yahoogimmickchamberhypnoticechounfoundedbonedunpainted", None, None).await?;

	/*client.register_event_handler(|ev: SyncRoomMessageEvent| async move {
		println!("Received a message {:?}", ev);
        //let to_gui = "{:?}";
	})
	.await;*/
    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    client.sync(SyncSettings::default()).await;


    //rum saker???
    let room = client.get_room(room_id!("!FVZaPevCZhhurovOAA:norrland.xyz"));

    let respond = "amogus";
    //få ut medelenden ur roomid :)))))
   /* Ok(matrixmsg {
        msg: "test".to_string(),
    }) */
    Ok(respond.to_string())
}
