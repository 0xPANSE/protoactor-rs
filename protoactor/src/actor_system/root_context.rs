use crate::actor::{Actor, Handler};
use crate::actor_ref::ActorRef;
use crate::actor_system::ActorSystemInner;
use crate::message::Message;
use crate::props::Props;
use std::sync::Arc;
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct RootContext {
    actor_system_inner: Arc<ActorSystemInner>,
}

impl RootContext {
    pub(crate) fn new(actor_system_inner: Arc<ActorSystemInner>) -> Self {
        RootContext { actor_system_inner }
    }

    pub fn spawn<A>(&self, props: Props<A>) -> ActorRef<A>
    where
        A: Actor,
    {
        todo!("Implement spawning logic here")
    }

    pub fn send<M, A>(&self, target: ActorRef<A>, msg: M) -> oneshot::Receiver<M::Result>
    where
        M: Message,
        A: Actor + Handler<M>,
    {
        todo!("Implement message sending logic here")
    }
}
