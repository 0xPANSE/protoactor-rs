mod root;

use crate::actor::{Actor, ActorRef};
pub use root::RootContext;

// The Context struct provides an actor with the necessary context to interact
// with the actor system, such as sending messages or spawning child actors.
pub struct Context<A: Actor> {
    // The ActorRef that represents the current actor.
    actor_ref: ActorRef<A>,
}

impl<A: Actor> Context<A> {
    pub fn new(actor_ref: ActorRef<A>) -> Self {
        Context { actor_ref }
    }
}
