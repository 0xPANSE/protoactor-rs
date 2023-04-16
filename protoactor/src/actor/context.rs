use crate::actor::{Actor, ActorRef, Handler};
use crate::actor_system::inner::ActorSystemInner;
use crate::mailbox::Mailbox;
use crate::message::Message;
use std::sync::{Arc, RwLock};

/// `Context` represents the runtime environment of an actor.
///
/// It provides a way to interact with the actor's state and manage its lifecycle.
/// The `Context` wraps an `Arc<Mutex<ContextInner<A>>>`, providing thread-safe access
/// to the inner context state. This struct also includes methods for working with remote
/// actors and sending messages between them.
pub struct Context<A: Actor> {
    inner: Arc<RwLock<ContextInner<A>>>,
}

impl<A> Context<A>
where
    A: Actor,
{
    /// Creates a new instance of `Context`.
    pub(crate) fn new(mailbox: Mailbox<A>, actor_system: Arc<RwLock<ActorSystemInner>>) -> Self {
        let sender = mailbox.get_sender();
        let inner = ContextInner {
            mailbox,
            actor_system: actor_system.clone(),
            actor: None,
            actor_ref: ActorRef::new(actor_system, sender),
        };
        Context {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub(crate) fn initialize(&mut self, actor: A) {
        let mut inner = self.inner.write().unwrap();
        inner.actor = Some(actor);
    }

    pub(crate) fn actor_ref(&self) -> ActorRef<A> {
        self.inner.read().unwrap().actor_ref.clone()
    }

    /// Sends a message to the specified actor.
    pub async fn send<M>(&self, target: ActorRef<A>, msg: M)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        let inner = self.inner.read().unwrap();
        inner.send(target, msg).await;
    }

    /// Responds to the sender of the current message. If sender is not available, this method
    /// does nothing except logging an warning.
    pub fn respond<M>(&self, msg: M) -> M::Result
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        todo!("Implement Context::respond")
    }

    /// Watches the specified actor. If the actor is terminated, the current actor will be
    /// notified.
    pub fn watch(&self, target: ActorRef<A>) {
        todo!("Implement Context::watch")
    }

    /// Unwatches the specified actor.
    pub fn unwatch(&self, target: ActorRef<A>) {
        todo!("Implement Context::unwatch")
    }

    /// Forwards the current message to the specified actor.
    /// The current message will be sent to the specified actor as is.
    pub fn forward<M>(&self, target: ActorRef<A>)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        todo!("Implement Context::forward")
    }

    /// Captures the current message or envelope for the context. Use this to stash the current
    /// message for later processing.
    pub fn stash(&self) {
        todo!("Implement Context::stash")
    }
}

/// `ContextInner` represents the inner state of a `Context`.
///
/// It can store any additional state needed for the context, such as a reference
/// to the actor or other components. This struct is used internally by `Context`.
pub struct ContextInner<A: Actor> {
    mailbox: Mailbox<A>,
    actor_system: Arc<RwLock<ActorSystemInner>>,
    actor: Option<A>,
    actor_ref: ActorRef<A>,
}

impl<A> ContextInner<A>
where
    A: Actor,
{
    /// Creates a new instance of `ContextInner` with the given actor.

    /// Sends a message to the specified actor using the actor's context.
    pub async fn send<M>(&self, target: ActorRef<A>, msg: M)
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        // Sending logic will be implemented here, including remote communication
        // when the target actor is on another node.
    }

    // Additional methods to interact with the inner context state can be added here.
}
