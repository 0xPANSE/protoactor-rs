use crate::actor::{Actor, ActorRef};
use std::cell::RefCell;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

pub enum ActorState {
    Started,
    Running,
    Stopping,
    Stopped,
}

pub trait ActorContext<A>
where
    A: Actor,
{
    fn actor_ref(&self) -> ActorRef<A>;

    fn set_stop_sender(&self, stop_sender: oneshot::Sender<()>);

    fn stop(&self);
}

pub struct Context<A>
where
    A: Actor,
{
    actor_ref: ActorRef<A>,
    stop_sender: RefCell<Option<Sender<()>>>,
}

impl<A> Context<A>
where
    A: Actor,
{
    pub(crate) fn new(actor_ref: ActorRef<A>) -> Self {
        Self {
            actor_ref,
            stop_sender: RefCell::new(None),
        }
    }

    pub fn self_(&self) -> ActorRef<A> {
        self.actor_ref.clone()
    }
}

impl<A> ActorContext<A> for Context<A>
where
    A: Actor,
{
    fn actor_ref(&self) -> ActorRef<A> {
        self.actor_ref.clone()
    }

    fn set_stop_sender(&self, stop_sender: oneshot::Sender<()>) {
        *self.stop_sender.borrow_mut() = Some(stop_sender);
    }

    fn stop(&self) {
        if let Some(stop_sender) = self.stop_sender.borrow_mut().take() {
            stop_sender.send(()).unwrap();
        }
    }
}
