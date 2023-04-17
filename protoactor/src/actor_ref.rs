use crate::actor::Handler;
use crate::actor_system::root_context::RootContext;
use crate::config::NO_HOST;
use crate::mailbox::MailboxSender;
use crate::message::{Message, MessageEnvelope};
use crate::prelude::Actor;
use crate::proto::Pid;
use uuid::Uuid;

/// The ActorRef struct is a reference to an actor process.
/// It holds the `Pid` that uniquely identifies the actor process and the `ActorProcess` that handles
/// the actual processing of messages for the actor. The tell method is used to send a message to
/// the actor process. The `Deref` trait is implemented to allow getting the `Pid` from an `ActorRef`.
pub struct ActorRef<A>
where
    A: Actor,
{
    pid: Pid,
    sender: MailboxSender<A>,
    root_context: RootContext,
    // _marker: PhantomData<A>,
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
    pub(crate) fn new(pid: Pid, root_context: RootContext, mailbox: MailboxSender<A>) -> Self {
        Self {
            pid,
            sender: mailbox,
            root_context,
            // _marker: PhantomData,
        }
    }

    pub(crate) fn new_named(
        name: String,
        sender: MailboxSender<A>,
        root_context: RootContext,
    ) -> Self {
        Self {
            pid: Pid::new(name, NO_HOST.to_string()),
            sender,
            root_context,
            // _marker: PhantomData,
        }
    }

    pub(crate) async fn send_user_message<M>(&self, envelope: MessageEnvelope<A, M>)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Actor + Handler<M>,
    {
        if let Err(e) = self.sender.send::<M>(Box::new(envelope)).await {
            log::error!("Failed to send message: {}", e);
        }
    }

    pub fn request_async<M>(&self, msg: M) -> tokio::sync::oneshot::Receiver<M::Result>
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let message_envelope = MessageEnvelope::new(msg, Some(tx));
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
            sender: self.sender.clone(),
            root_context: self.root_context.clone(),
            // _marker: PhantomData,
        }
    }
}
