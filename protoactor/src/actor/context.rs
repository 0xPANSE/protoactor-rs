use crate::actor::Actor;
use crate::actor_ref::ActorRef;
use crate::actor_system::ActorSystem;
use crate::prelude::Props;

/// The Context trait represents the runtime context of an actor, providing access
/// to actor-related functionality like spawning child actors, watching other actors,
/// and stopping actors.
pub trait Context<A: Actor> {
    /// Returns the actor reference associated with this context.
    /// This reference can be used to send messages to the actor.
    fn actor_ref(&self) -> ActorRef<A>;

    /// Spawns a new actor using the given props and returns its actor reference.
    /// The spawned actor will be a child of the current actor.
    fn spawn<B>(&self, props: Props<B>) -> ActorRef<B>
    where
        B: Actor;

    /// Stops the actor associated with the given actor reference.
    /// This will trigger the actor's stopping lifecycle events and
    /// eventually terminate the actor.
    fn stop(&self, actor_ref: &ActorRef<A>);

    /// Adds the actor associated with the given actor reference to the watch list.
    /// The watching actor will receive a terminated message if the watched actor is terminated.
    fn watch(&self, actor_ref: &ActorRef<A>);

    /// Removes the actor associated with the given actor reference from the watch list.
    /// The watching actor will no longer receive a terminated message if the watched actor is terminated.
    fn unwatch(&self, actor_ref: &ActorRef<A>);
}

/// The ContextImpl struct holds the actor's state and implements the Context trait.
/// It provides the runtime context for the actor, allowing it to interact with the
/// actor system and other actors.
pub struct ContextImpl<A: Actor> {
    /// The actor reference associated with this context.
    /// This reference can be used to send messages to the actor.
    actor_ref: ActorRef<A>,
    /// A reference to the actor system that manages the actors.
    /// The actor system provides facilities for managing the lifecycle of actors
    /// and sending messages between them.
    actor_system: ActorSystem,
}

/// Implementing the Context trait for ContextImpl.
impl<A: Actor> Context<A> for ContextImpl<A> {
    fn actor_ref(&self) -> ActorRef<A> {
        todo!()
    }

    fn spawn<B>(&self, props: Props<B>) -> ActorRef<B>
    where
        B: Actor,
    {
        todo!()
    }

    fn stop(&self, actor_ref: &ActorRef<A>) {
        todo!()
    }

    fn watch(&self, actor_ref: &ActorRef<A>) {
        todo!()
    }

    fn unwatch(&self, actor_ref: &ActorRef<A>) {
        todo!()
    }
}
