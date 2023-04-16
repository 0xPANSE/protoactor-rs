use crate::actor::Handler;
use crate::actor_system::inner::ActorSystemInner;
use crate::mailbox::MailboxSender;
use crate::message::{Envelope, Message, MessageEnvelope};
use crate::prelude::Actor;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// The ActorRef struct is a reference to an actor process.
/// It holds the `Pid` that uniquely identifies the actor process and the `ActorProcess` that handles
/// the actual processing of messages for the actor. The tell method is used to send a message to
/// the actor process. The `Deref` trait is implemented to allow getting the `Pid` from an `ActorRef`.
pub struct ActorRef<A: Actor> {
    id: Uuid,
    sender: MailboxSender<A>,
    actor_system: Arc<RwLock<ActorSystemInner>>,
    _marker: PhantomData<A>,
}

impl<A: Actor> ActorRef<A> {
    pub(crate) fn new(
        actor_system: Arc<RwLock<ActorSystemInner>>,
        mailbox: MailboxSender<A>,
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            sender: mailbox,
            actor_system,
            _marker: PhantomData,
        }
    }

    pub(crate) fn new_named(
        actor_system: Arc<RwLock<ActorSystemInner>>,
        name: String,
        sender: MailboxSender<A>,
    ) -> Self {
        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());
        Self {
            id,
            sender,
            actor_system,
            _marker: PhantomData,
        }
    }

    pub async fn send_user_message<M>(&self, envelope: MessageEnvelope<A, M>)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        if let Err(e) = self.sender.send::<M>(Box::new(envelope)).await {
            log::error!("Failed to send message: {}", e);
        }
    }
}

impl<A: Actor> Clone for ActorRef<A> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            sender: self.sender.clone(),
            actor_system: Arc::clone(&self.actor_system),
            _marker: PhantomData,
        }
    }
}
