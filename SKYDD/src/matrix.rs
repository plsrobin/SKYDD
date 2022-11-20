use super::*;

use iced_native::{subscription::events, widget::column};
//imports
//Matrix imports
use matrix_sdk::{
    Client, config::SyncSettings, room::MessagesOptions,
    ruma::{room_id, user_id, api::client::membership::joined_rooms,}, ClientBuildError, HttpError,
};

use iced::{Length};
use iced::{Column, Row, Text};



//eror handling

//matrix general errors?
impl From<matrix_sdk::Error> for Error {
    fn from(error: matrix_sdk::Error) -> Error {
        dbg!(error);
        Error::MatrixError
    }
}
//matrix client build errors
impl From<matrix_sdk::ClientBuildError> for Error {
    fn from(error: ClientBuildError) -> Error {
        dbg!(error);
        Error::ClientBuildError
    }

}
//implements http error from matrix
impl From<matrix_sdk::HttpError> for Error {
    fn from(error: HttpError) -> Error {
        dbg!(error);
        Error::HttpError
    }
}
//implements parseError
impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        dbg!(error);
        Error::ParseError
    }
}



#[derive(Debug, Clone)]
pub struct MatrixMsg {
    //msg: String,
    msg: Vec<matrix_sdk::deserialized_responses::TimelineEvent>,
    joined_rms: Vec<matrix_sdk::room::Joined>,
}

impl MatrixMsg {
 
    //returns all joined rooms
    pub fn rms(&mut self) -> Element<Message> {
        let rooms = self.joined_rms.iter().fold(
            Column::new().spacing(10),
            | column, joined | {
                column.push(Text::new(format!("{:?}", joined.name())))
            }
        );

        Column::new()
            .push(rooms)
            .into()
    }

    //returns messages of room
    pub fn view(&mut self) -> Element<Message> {
        
        let events = self.msg.iter().rev().fold(
            Column::new().spacing(10),
            | column, event | {
                //column.push(Text::new(format!("{:?}", event)).size(12))
                //column.push(Text::new(format!("{}", event.event.json().to_string().split("\",\"msgtype").take(1).collect::<Vec<_>>()[0].to_string().split("body\":\"").take(2).collect::<Vec<_>>()[0])).size(24))
                column.push(Text::new(format!("{}", event.event.json().to_string().split("\",\"msgtype").take(1).collect::<Vec<_>>()[0].to_string().split("body\":\"").nth(1).unwrap().to_string().split("\",\"mimetype" ).nth(0).unwrap().to_string().split("\"},\"origin_server_ts").nth(0).unwrap().to_string())).size(24))
            }
        );  

       Column::new()
           .push(events)
           .into() 
    }
    pub async fn search_msg() -> anyhow::Result<MatrixMsg, Error> {
	    let userid = user_id!("@testuser3:norrland.xyz");
	    let client = Client::builder().user_id(userid).build().await?;
        client.login(userid, "yahoogimmickchamberhypnoticechounfoundedbonedunpainted", None, None).await?;

		//initial sync, uuuhhh
        println!("starting sync_once");
		client.sync_once(SyncSettings::default()).await.unwrap();

        println!("last step...");

        let room = client.get_room(room_id!("!FVZaPevCZhhurovOAA:norrland.xyz")).unwrap();
        let mut options = MessagesOptions::new(matrix_sdk::ruma::api::client::message::get_message_events::v3::Direction::Backward);
        //let message = room.messages(options).await.unwrap().chunk.first().unwrap().event.json().to_string();
        let message = room.messages(options).await.unwrap().chunk;

        //get joined rooms
        let rooms = client.joined_rooms();

        Ok(MatrixMsg {
            msg: message,
            joined_rms: rooms,
        })
    }
}
