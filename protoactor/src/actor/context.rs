use crate::actor::{Actor, ActorRef};
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

    pub fn sender<B>(&self) -> Option<&ActorRef<B>>
    where
        B: Actor,
    {
        self.sender
            .as_ref()
            .and_then(|sender| sender.downcast_ref::<ActorRef<B>>())
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
