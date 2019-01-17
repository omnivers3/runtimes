extern crate sink;
extern crate omnivers3_systems_actor;

pub use omnivers3_systems_actor::{ IntoActorSystem };

pub mod console;
pub mod linereader;
pub mod mocklinereader;

#[derive(Clone, Debug, PartialEq)]
pub enum StdinCommands {
    Initialize,
    Await,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StdinEvents {
    Initialized,
    Waiting,
    LineReceived (usize, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StdinErrors {
    AlreadyInitialized,
    NotInitialized,
    IoError (String),
}
