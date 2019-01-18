use std::iter::Iterator;
use std::time::{ Duration };
use std::thread;

use omnivers3_systems_actor::{ Actor, ActorState };
use sink::*;

use super::*;
use super::StdinCommands::*;
use super::StdinEvents::*;
use super::StdinErrors::*;

#[derive(Clone, Debug)]
pub struct Config {
    /// Set of values to emit when await command is executed
    values: Vec<String>,
    /// Simulate a constant delay between responses
    delay_ms: u32,
    /// True causes the thread to park emulating a permanent block on user input
    auto_halt: bool,
}

impl Config {
    pub fn new<TSource>(source: TSource) -> Self
    where
        TSource: IntoIterator,
        TSource::Item: ToString,
    {
        let mut values = vec![
            "foo".to_owned(),
            "bar".to_owned(),
            "baz".to_owned(),
        ];
        let mut source: Vec<String> = source
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        values.append(&mut source);
        let values = values.iter().map(|i| format!{"{}\n", i}).collect();
        
        Config {
            values,
            delay_ms: 1000,
            auto_halt: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    values: Vec<String>,
    index: usize,
    initialized: bool,
}

impl ActorState<Config> for State {
    fn from(config: &Config) -> Self {
        State {
            values: config.values.clone(),
            index: 0,
            initialized: false,
        }
    }
}

impl Iterator for State {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.values.len() <= self.index {
            None
        } else {
            let result = self.values[self.index].to_owned();
            self.index += 1;
            Some (result)
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
        match (command, state.initialized) {
            (Initialize, true) => {
                events.send(Err(AlreadyInitialized))
            }
            (Initialize, false) => {
                state.initialized = true;
                events.send(Ok(Initialized));
            }
            (Await, false) => {
                events.send(Err(NotInitialized))
            }
            (Await, true) => {
                events.send(Ok(Waiting));
                thread::sleep(Duration::from_millis(self.delay_ms.into()));
                if let Some (line) = state.next() {
                    events.send(Ok(LineReceived (line.len(), line)));
                } else {
                    if self.auto_halt {
                        thread::park();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::{ RefCell };

    use sink::fnsink::{ FnSink };

    #[test]
    fn should_dispatch_already_initialized_when_initialize_called_twice() {
        let expected = vec![
            Ok (StdinEvents::Initialized),
            Err (StdinErrors::AlreadyInitialized),
        ];
        let actual = RefCell::new(vec![]);
        let event_sink = FnSink::new(|e: Result<StdinEvents, StdinErrors>| {
            let mut prev = actual.borrow_mut();
            (*prev).push(e);
        });

        let reader = Config::new(&["..."]);

        let system = reader.bind(&event_sink);

        system.send(StdinCommands::Initialize);
        system.send(StdinCommands::Initialize);

        let actual = actual.borrow();
        assert_eq!(expected.len(), actual.len());
        for i in 0..actual.len() {
            assert_eq!(expected[i], actual[i], "failed at {}", i);
        }
    }
}