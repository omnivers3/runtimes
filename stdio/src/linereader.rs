use std::io;
use std::io::{ BufRead };//, Stdin, Stdout, Write };

use omnivers3_systems_actor::{ Actor, ActorState };
use sink::*;

use super::*;
use super::StdinCommands::*;
use super::StdinEvents::*;
use super::StdinErrors::*;

#[derive(Debug)]
pub struct State {
    stdin: Option<io::Stdin>,
}

#[derive(Clone, Debug)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Config {}
    }
}

impl ActorState<Config> for State {
    fn from(_config: &Config) -> Self {
        State {
            stdin: None,
        }
    }
}

impl Actor for Config {
    type TState = State;
    type TCommands = StdinCommands;
    type TEvents = Result<StdinEvents, StdinErrors>;
    type TResult = ();

    fn handle(&self,
        state: &mut Self::TState,
        command: Self::TCommands,
        events: &impl Sink<TInput=Self::TEvents, TResult=()>
    ) -> Self::TResult {
        if let Some (ref mut stdin) = state.stdin {
            match command {
                Initialize => {
                    events.send(Err(AlreadyInitialized));
                }
                Await => {
                    events.send(Ok(Waiting));
                    let mut buffer = String::default();
                    match { stdin.lock().read_line(&mut buffer) } {
                        Ok (len) => {
                            events.send(Ok(LineReceived (len, buffer)));
                        }
                        Err (err) => {
                            events.send(Err(IoError (format!("{:?}", err))));
                        }
                    }
                }
            }
        } else {
            match command {
                Initialize => {
                    state.stdin = Some(io::stdin());
                    events.send(Ok(Initialized));
                }
                Await => {
                    events.send(Err(NotInitialized));
                }
            }
        }
    }
}