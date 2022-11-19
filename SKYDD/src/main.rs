//iced imports, used to start gui.rs
use iced::{Application, Element, Settings};

//imports gui.rs
use crate::gui::Gui;
use crate::gui::Message;
mod gui;

//imports matrix.rs
use crate::matrix::MatrixMsg;
mod matrix;

//main
#[tokio::main]
async fn main() -> iced::Result {
    Gui::run(Settings::default())
}

//error handling, used across modules
#[derive(Debug, Clone)]
pub enum Error {
    //matrix errors
    ClientBuildError,
    MatrixError,
    ParseError,
    HttpError,
}
