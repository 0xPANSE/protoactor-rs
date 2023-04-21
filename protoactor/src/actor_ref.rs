use crate::actor::Handler;
use crate::actor_system::root_context::RootContext;
use crate::config::NO_HOST;
use crate::mailbox::MailboxSender;
use crate::message::{Message, MessageEnvelope};
use crate::prelude::Actor;
use crate::proto::Pid;
use std::fmt::Debug;

/// The ActorRef struct is a reference to an actor process.
/// It holds the `Pid` that uniquely identifies the actor process and the `ActorProcess` that handles
/// the actual processing of messages for the actor. The tell method is used to send a message to
/// the actor process. The `Deref` trait is implemented to allow getting the `Pid` from an `ActorRef`.
pub struct ActorRef<A>
where
    A: Actor,
{
    pid: Pid,
    pub(crate) mailbox_sender: MailboxSender<A>,
    root_context: RootContext,
}

impl<A> PartialEq for ActorRef<A>
where
    A: Actor,
{
    fn eq(&self, other: &Self) -> bool {
        self.pid.id == other.pid.id && self.pid.address == other.pid.address
    }
}

impl<A: Actor> Eq for ActorRef<A> {}

impl<A: Actor> ActorRef<A> {
    pub(crate) fn new(
        pid: Pid,
        root_context: RootContext,
        mailbox_sender: MailboxSender<A>,
    ) -> Self {
        Self {
            pid,
            mailbox_sender,
            root_context,
        }
    }

    pub(crate) async fn send_user_message<M>(&self, envelope: MessageEnvelope<A, M>)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Actor + Handler<M>,
    {
        if let Err(e) = self.mailbox_sender.send::<M>(Box::new(envelope)).await {
            log::error!("Failed to send message: {}", e);
        }
    }

    pub fn request<M>(&self, msg: M) -> tokio::sync::oneshot::Receiver<M::Result>
    where
        M: Message + Send + 'static,
        M::Result: Debug + Send + 'static,
        A: Handler<M>,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let sender_ref = SenderRef::new(Box::new(move |res| {
            tx.send(res).unwrap();
        }));
        let message_envelope = MessageEnvelope::new(msg, Some(sender_ref));
        self.send_user_message(message_envelope);
        rx
    }

    pub fn id(&self) -> String {
        self.pid.id.clone()
    }

    pub fn address(&self) -> String {
        self.pid.address.clone()
    }

    pub fn pid(&self) -> Pid {
        self.pid.clone()
    }
}

impl<A: Actor> Clone for ActorRef<A> {
    fn clone(&self) -> Self {
        Self {
            pid: self.pid.clone(),
            mailbox_sender: self.mailbox_sender.clone(),
            root_context: self.root_context.clone(),
        }
    }
}

pub struct SenderRef<M>
where
    M: Message + Send + 'static,
    M::Result: Send + 'static,
{
    respond_fn: Box<dyn FnOnce(M::Result) + Send + 'static>,
}

impl<M> SenderRef<M>
where
    M: Message + Send + 'static,
    M::Result: Send + 'static,
{
    pub fn new(respond_fn: Box<dyn FnOnce(M::Result) + Send + 'static>) -> Self {
        Self { respond_fn }
    }

    pub fn respond(self, result: M::Result) {
        (self.respond_fn)(result)
    }
}
