use std::any::Any;
use std::future::Future;
use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::oneshot::error::RecvError;

use crate::actor::{Actor, Handler};
use crate::actor_system::root_context::RootContext;
use crate::mailbox::MailboxSender;
use crate::message::{Message, MessageEnvelope};
use crate::proto::Pid;

/// The ActorRef struct is a reference to an actor process.
/// It holds the `Pid` that uniquely identifies the actor process and the `ActorProcess` that handles
/// the actual processing of messages for the actor. The tell method is used to send a message to
/// the actor process. The `Deref` trait is implemented to allow getting the `Pid` from an `ActorRef`.
pub struct ActorRef<A>
where
    A: Actor,
{
    pid: Pid,
    mailbox_sender: Arc<MailboxSender<A>>,
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
        mailbox_sender: Arc<MailboxSender<A>>,
    ) -> Self {
        Self {
            pid,
            mailbox_sender,
            root_context,
        }
    }

    pub(crate) fn send_user_message<M>(&self, envelope: MessageEnvelope<A, M>)
    where
        M: Message + Send + 'static,
        A: Actor + Handler<M>,
    {
        if let Err(e) = self.mailbox_sender.send::<M>(Box::new(envelope)) {
            log::error!("Failed to send message: {}", e);
        }
    }

    pub fn request<M, R>(&self, msg: M) -> impl Future<Output = Result<Box<R>, RecvError>>
    where
        M: Message + Send + 'static,
        R: Message + Send + 'static,
        A: Handler<M>,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let sender_ref = SenderRef::new(Box::new(move |res| {
            tx.send(res).unwrap();
        }));
        let message_envelope = MessageEnvelope::new(msg, Some(sender_ref));
        self.send_user_message(message_envelope);
        rx.then(|res| async { res.map(|res| res.downcast().unwrap()) })
    }

    pub fn send<M>(&self, msg: M)
    where
        M: Message + Send + 'static,
        A: Handler<M>,
    {
        // send message to actor using root context
        let message_envelope = MessageEnvelope::new(msg, None);
        self.send_user_message(message_envelope);
    }

    pub fn id(&self) -> &str {
        self.pid.id.as_ref()
    }

    pub fn address(&self) -> &str {
        self.pid.address.as_ref()
    }

    pub fn pid(&self) -> &Pid {
        &self.pid
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

pub struct SenderRef {
    respond_fn: Box<dyn FnOnce(Box<dyn Any + Send + 'static>) + Send + 'static>,
}

impl SenderRef {
    pub fn new(
        respond_fn: Box<impl FnOnce(Box<dyn Any + Send + 'static>) + Send + 'static>,
    ) -> Self {
        Self { respond_fn }
    }

    pub fn respond<M>(self, result: M)
    where
        M: Message + Send + 'static,
    {
        (self.respond_fn)(Box::new(result))
    }
}
