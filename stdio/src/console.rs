use omnivers3_systems_actor::{ Actor, ActorState };
use sink::*;

use super::*;
use super::StdinCommands::*;

#[derive(Clone, Debug)]
pub struct State {}

#[derive(Clone, Debug)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Config {}
    }
}

impl ActorState<Config> for State {
    fn from(_config: &Config) -> Self {
        State {}
    }
}

impl Actor for Config {
    type TState = State;
    type TCommands = ();
    type TEvents = StdinCommands;
    type TResult = ();

    fn handle(&self,
        _state: &mut Self::TState,
        _command: (),
        events: &impl Sink<TInput=Self::TEvents, TResult=()>
    ) -> Self::TResult {
        // let mut count = 2;
        events.send(Initialize);
        loop {
            // if count <= 0 {
            //     break;
            // }
            // count -= 1;
            events.send(Await);
        }
    }
}