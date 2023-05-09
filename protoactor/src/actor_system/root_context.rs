use crate::actor::{Actor, Context, Handler};
use crate::actor_ref::{ActorRef, SenderRef};
use crate::actor_system::{ActorSystem, ActorSystemInner};
use crate::message::{Message, MessageEnvelope};
use crate::props::Props;
use log::{info, warn};
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct RootContext {
    actor_system: Arc<ActorSystemInner>,
}

impl RootContext {
    pub(crate) fn schedule(&self, p0: impl Future<Output = ()> + Send + 'static) {
        // use tokio to spawn a future
        tokio::spawn(p0);
    }
}

impl RootContext {
    pub(crate) fn new(actor_system: Arc<ActorSystemInner>) -> Self {
        RootContext { actor_system }
    }

    pub fn from_system(system: ActorSystem) -> Self {
        let actor_system = system.inner;
        RootContext { actor_system }
    }

    pub fn spawn<A>(&self, props: &Props<A>) -> ActorRef<A>
    where
        A: Actor<Context = Context<A>> + Send + Sync + 'static,
    {
        self.actor_system.spawn(props, self.clone())
    }

    pub fn spawn_named<A>(&self, name: &str, props: &Props<A>) -> ActorRef<A>
    where
        A: Actor<Context = Context<A>>,
    {
        self.actor_system
            .spawn_named(name.to_string(), props, self.clone())
    }

    pub async fn send<M, A>(&self, target: &ActorRef<A>, msg: M)
    where
        M: Message + Send + 'static,
        A: Actor + Handler<M>,
    {
        let (sender, receiver) = oneshot::channel();
        let sender_ref = SenderRef::new(Box::new(|res| sender.send(res).unwrap()));
        let envelope: MessageEnvelope<A, M> = MessageEnvelope::new(msg, Some(sender_ref));
        target.send_user_message(envelope).await
    }
}
