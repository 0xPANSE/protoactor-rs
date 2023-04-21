use crate::actor::{Actor, ActorRef};
use crate::actor_ref::SenderRef;
use crate::message::{Message, MessageEnvelope};
use crate::prelude::Handler;
use futures::SinkExt;
use std::any::Any;
use std::cell::RefCell;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

pub enum ActorState {
    Started,
    Running,
    Stopping,
    Stopped,
}

pub trait ActorContext {
    fn set_stop_sender(&self, stop_sender: Sender<()>);

    fn stop(&self);
}

pub struct Context<A>
where
    A: Actor,
{
    myself: ActorRef<A>,
    sender: Option<Box<dyn Any + Send>>,
    stop_channel: RefCell<Option<Sender<()>>>,
}

impl<A> Context<A>
where
    A: Actor,
{
    pub(crate) fn new(actor_ref: ActorRef<A>) -> Self {
        Self {
            myself: actor_ref,
            sender: None,
            stop_channel: RefCell::new(None),
        }
    }

    pub fn myself(&self) -> &ActorRef<A> {
        &self.myself
    }

    pub(crate) fn set_sender(&mut self, sender: Box<dyn Any + Send>) {
        self.sender = Some(sender);
    }

    pub fn sender<B, R>(&self) -> Option<&ActorRef<B>>
    where
        B: Actor + Handler<R>,
        R: Message + Send + 'static,
        R::Result: Message + Send + 'static,
    {
        self.sender
            .as_ref()
            .and_then(|sender| sender.downcast_ref::<ActorRef<B>>())
    }

    /// Send a message to the actor and wait for the response.
    /// This method is only available if the actor implements the `Handler` trait for the message type.
    /// If the actor does not implement the `Handler` trait for the message type, this method will panic.
    /// If the actor is not running, this method will panic.
    pub fn response<S, M>(&self, response: M)
    where
        S: Actor + Handler<M> + Send + 'static,
        M: Message + Send + 'static,
        M::Result: Send + 'static,
    {
        if let Some(sender) = self.sender::<S, M>().take() {
            // let myself = self.myself.mailbox_sender.clone();
            // let sender_ref = SenderRef::new(Box::new(|r| {
            //     tokio::spawn(async move {
            //         myself.send(Box::new(r)).await;
            //     });
            // }));
            let envelope = MessageEnvelope::new(response, None);
            tokio::spawn(async move {
                sender.send_user_message(envelope).await;
            });
        } else {
            panic!("No sender found");
        }
    }
}

impl<A> ActorContext for Context<A>
where
    A: Actor,
{
    fn set_stop_sender(&self, stop_sender: oneshot::Sender<()>) {
        *self.stop_channel.borrow_mut() = Some(stop_sender);
    }

    fn stop(&self) {
        if let Some(stop_sender) = self.stop_channel.borrow_mut().take() {
            stop_sender.send(()).unwrap();
        }
    }
}
