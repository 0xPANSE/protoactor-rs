use crate::actor::Actor;
use crate::mailbox::Mailbox;
use std::marker::PhantomData;
use std::sync::Arc;

/// The `ActorProcess` is a component responsible for managing the lifecycle of an actor,
/// handling incoming messages, and maintaining the actor's state.
/// The main responsibilities of the ActorProcess include:
///
/// * Managing the actor's mailbox and processing messages.
/// * Calling the actor's handle method to process messages.
/// * Handling the actor's lifecycle events, such as started, stopping, and stopped.
pub enum ActorProcess<A>
where
    A: Actor,
{
    Local(LocalActorProcess<A>),
    Remote(RemoteActorProcess<A>),
}

impl<A: Actor> ActorProcess<A> {
    pub fn new_local(actor: A, mailbox: Mailbox<A>) -> Self {
        let local_actor_process = LocalActorProcess {
            actor: Arc::new(actor),
            mailbox,
        };
        ActorProcess::Local(local_actor_process)
    }
}

pub struct LocalActorProcess<A>
where
    A: Actor,
{
    actor: Arc<A>,
    mailbox: Mailbox<A>,
}

pub struct RemoteActorProcess<A: Actor> {
    // todo: implement remote actor process later, below is just a placeholder
    remote_address: String,
    _marker: PhantomData<A>,
}
