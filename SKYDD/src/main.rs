//BLock_on?? behöver för async i thread!?!?!?
use futures::executor::block_on;
//test
use tokio::time::{sleep, Duration};

//Matrix någonting?
use std::{convert::TryFrom, str::SplitAsciiWhitespace};
use matrix_sdk::{
    Client, config::SyncSettings, 
    ruma::{room_id, server_name, user_id, events::room::message::SyncRoomMessageEvent, api::client::room::get_room_event},
};

//iced (gui) imports
//use iced::widget::{container};
use iced::{executor, button, Button, Application, Command, Element, Settings, Text, Container, Length, Column};

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
    async fn search_msg() -> Result<matrixmsg, Error> {
        //let response = matrixtest();
        Ok(matrixmsg {
            //msg: response.unwrap().to_string(),
            msg: "snälla fungera".to_string(),
        })
    }
}

async fn icedtest() -> iced::Result  {
    Hello::run(Settings::default())
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

	client.register_event_handler(|ev: SyncRoomMessageEvent| async move {
		println!("Received a message {:?}", ev);
        //let to_gui = "{:?}";
	})
	.await;
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
