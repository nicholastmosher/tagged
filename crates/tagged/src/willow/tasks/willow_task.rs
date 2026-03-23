// I want to somehow generalize the following:
//
// - Weave in the central "async context" object
// - Make task events more flexible, rather than finite via enums

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use zed::unstable::{
    db::smol::stream::{Stream, StreamExt as _, empty},
    gpui::{self, Action, actions},
};

/// External API, handle to send/receive events to the spawned WillowTask
pub struct WillowTask {
    //
}

/// Internal API, available to be used by task implementors
pub struct WillowTaskContext {
    //
    handlers: HashMap<TypeId, Arc<dyn Handler + Send + Sync>>,
}

/// All incoming events are variants of this input event
pub enum WillowTaskInput {
    Shutdown,
}

// hold a collection of Arc<dyn Handler>
// handlers keyed by event type and state instance?

pub trait Handler {
    fn handle_event(&self) {}
}

pub trait WillowTaskEvent: Any {
    //
}

impl WillowTask {
    pub fn spawn() {
        //
        let future = WillowTaskContext::new().run();
        let tokio_handle = tokio::spawn(future);
    }
}

impl WillowTaskContext {
    pub fn new() -> Self {
        todo!()
    }

    fn create_task_stream() -> impl 'static + Stream<Item = WillowTaskInput> {
        empty()
    }

    pub async fn run(mut self) {
        //
        let mut task_stream = Self::create_task_stream();

        loop {
            let result = self.try_run(&mut task_stream).await;
            if let Err(error) = result {
                tracing::error!(?error, "WillowTask threw an error");
            }
        }
    }

    async fn try_run(
        &mut self,
        input: &mut (impl Unpin + Stream<Item = WillowTaskInput>),
    ) -> Result<()> {
        while let Some(input) = input.next().await {
            //
        }
        Ok(())
    }

    async fn try_handle_input(&mut self, input: WillowTaskInput) -> Result<()> {
        match &input {
            WillowTaskInput::Shutdown => {
                //
            }
        }

        Ok(())
    }
}

/// The "control plane" for the async universe of this application
///
/// - Operates as a state machine over a stream of input events.
/// - Event handling by dynamic dispatch to one/many (?) handlers
/// - State access similar to `Entity<T>`, but with some synchronization
///
/// This context will have a control task which accepts event inputs via
/// an async stream. Data-heavy operations should be handled elsewhere.
///
/// The control stream should be a pluggable interface similar to
/// the App pattern.
///
struct AsyncCx {
    // Experiment: use actions
    input_stream: Box<dyn Stream<Item = Box<dyn Any>>>,
    state: HashMap<TypeId, Box<dyn Any>>,
    handlers: HashMap<TypeId, Vec<Box<dyn Fn(&mut Self, &dyn Action)>>>,
}

// External API, cheap to clone, used anywhere to send events in?
struct AsyncCxHandle {
    //
}

impl AsyncCx {
    //

    fn add_handler<T: Action>(&mut self, f: impl 'static + Fn(&mut Self, &T)) {
        // self.state.entry(TypeId::of::<T>()).or_insert(default)
        let action_handler = move |this: &mut Self, action: &dyn Action| {
            if let Some(it) = action.as_any().downcast_ref::<T>() {
                f(this, it)
            }
        };

        self.handlers
            .entry(TypeId::of::<T>())
            .or_insert_with(Vec::new)
            .push(Box::new(action_handler));
    }
}

impl AsyncCxHandle {
    pub fn dispatch<T: Action>(&self, it: T) {
        //
    }
}

actions!([FooAction]);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Action)]
struct BarAction {
    //
    name: String,
}

fn it(mut it: AsyncCx) {
    it.add_handler(|this, action: &FooAction| {
        //
    });
}
