use super::*;

//imports
//Matrix imports
use matrix_sdk::{
    Client, config::SyncSettings, room::MessagesOptions,
    ruma::{room_id, user_id,}, ClientBuildError, HttpError,
};



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
    msg: String,
}

impl MatrixMsg {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Text::new(&self.msg)
                    .size(12)
                    .width(Length::Fill),
                )
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
        let options = MessagesOptions::new(matrix_sdk::ruma::api::client::message::get_message_events::v3::Direction::Backward);
        let message = room.messages(options).await.unwrap().chunk.first().unwrap().event.json().to_string();
        Ok(MatrixMsg {
            msg: message,
        })
    }
}
