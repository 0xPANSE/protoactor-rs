use crate::actor_ref::ActorRef;
use crate::message::Message;

mod actor_cell;
mod context;
mod handler;

use crate::actor::context::ActorContext;
pub(crate) use actor_cell::ActorCell;
pub use context::Context;
pub use handler::Handler;

/// Actor is a trait that should be implemented by all actors in the system.
/// This trait provides a `receive` method that actors use to handle incoming messages.
pub trait Actor: Sized + Send + 'static {
    /// `Actor::Context` is an associated type within the `Actor` trait that represents the execution
    /// context of an actor. The context provides the actor with access to various functionalities,
    /// such as spawning child actors, stopping the actor, and sending messages to other actors.
    /// By allowing actors to define their own context types, the Actor trait becomes more flexible
    /// and can accommodate different use cases and execution models.
    type Context: ActorContext<Self>;

    /// The method called when the actor starts. This method is called after the actor is created
    /// and before the actor starts processing messages. If not implemented, the default implementation
    /// in `Actor::started()` does nothing.
    /// # Example
    /// ```
    /// use protoactor::prelude::*;
    ///
    /// struct MyActor;
    ///
    /// impl Actor for MyActor {
    ///    type Context = Context<Self>;
    ///
    ///   fn started(&mut self, ctx: &mut Self::Context) {
    ///      println!("MyActor started");
    ///  }
    /// }
    /// ```
    fn started(&mut self, _ctx: &mut Self::Context) {}

    /// The method called when the actor stops. This method is called after the actor stops processing
    /// messages and before the actor is dropped. `Actor::stopped()` is called regardless of whether
    /// the actor stopped gracefully or due to an error.
    ///
    /// If not implemented, the default implementation does nothing.
    /// # Example
    /// ```no_run
    /// use protoactor::prelude::*;
    ///
    /// struct MyActor;
    ///
    /// impl Actor for MyActor {
    ///     type Context = Context<Self>;
    ///    
    ///     fn stopped(&mut self, ctx: &mut Self::Context) {
    ///         println!("MyActor stopped");
    ///     }
    /// }
    /// ```
    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

pub struct MessageResult<M: Message>(pub M);

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::Select;

    // A simple example of an actor, which has a name as its state.
    pub struct SampleActor {
        pub name: String,
    }

    // Implement the Actor trait for SampleActor.
    impl Actor for SampleActor {
        type Context = Context<Self>;

        // Print a message when the SampleActor starts.
        fn started(&mut self, _ctx: &mut Self::Context) {
            println!("SampleActor started");
        }
    }

    // A simple message that represents saying "Hi".
    pub struct HiMsg;

    // Implement the Message trait for HiMsg with a unit result type.
    impl Message for HiMsg {
        type Result = ();
    }

    // A simple message that represents saying "Bye".
    pub struct ByeMsg;

    // Implement the Message trait for ByeMsg with a unit result type.
    impl Message for ByeMsg {
        type Result = ();
    }

    // Implement the Handler trait for HiMsg, so SampleActor can process HiMsg.
    impl Handler<HiMsg> for SampleActor {
        fn handle(&mut self, _msg: HiMsg, _ctx: &mut Self::Context) {
            println!("Hi from {}", self.name);
        }
    }

    // Implement the Handler trait for ByeMsg, so SampleActor can process ByeMsg.
    impl Handler<ByeMsg> for SampleActor {
        fn handle(&mut self, _msg: ByeMsg, _ctx: &mut Self::Context) {
            println!("Bye from {}", self.name);
        }
    }

    // #[test]
    // fn test_sample_actor() {
    //     let mut sample_actor = SampleActor {
    //         name: "Alice".to_string(),
    //     };
    //
    //
    //
    //     // Manually call handle method for HiMsg
    //     Handler::<HiMsg>::handle(&mut sample_actor, HiMsg, &mut ());
    //     // Manually call handle method for ByeMsg
    //     Handler::<ByeMsg>::handle(&mut sample_actor, ByeMsg, &mut ());
    //
    //     // this call will not compile because the actor does not implement the Handler trait for the message type
    //     // Handler::<String>::handle(&mut sample_actor, "Hello".to_string(), &mut ());
    // }
}
