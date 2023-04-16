use crate::actor::{Actor, Context, Handler};
use crate::actor_ref::ActorRef;
use std::marker::PhantomData;

/// The `ActorProcess` is a component responsible for managing the lifecycle of an actor,
/// handling incoming messages, and maintaining the actor's state.
/// The main responsibilities of the ActorProcess include:
///
/// * Managing the actor's mailbox and processing messages.
/// * Calling the actor's handle method to process messages.
/// * Handling the actor's lifecycle events, such as started, stopping, and stopped.
pub enum ActorProcess<A: Actor> {
    Local(LocalActorProcess<A>),
    Remote(RemoteActorProcess<A>),
}

pub struct LocalActorProcess<A: Actor> {
    context: Context<A>,
    actor_ref: ActorRef<A>,
}

pub struct RemoteActorProcess<A: Actor> {
    // todo: implement remote actor process later, below is just a placeholder
    remote_address: String,
    _marker: PhantomData<A>,
}
