use crate::actor::{Actor, Context, Handler};
use crate::actor_ref::ActorRef;
use crate::actor_system::{ActorSystem, ActorSystemInner};
use crate::mailbox::Mailbox;
use crate::message::{Message, MessageEnvelope};
use crate::props::Props;
use std::sync::{Arc, RwLock};
use tokio::sync::oneshot;

pub struct RootContext {
    actor_system: Arc<RwLock<ActorSystemInner>>,
}

impl RootContext {
    pub(crate) fn new(actor_system: Arc<RwLock<ActorSystemInner>>) -> Self {
        RootContext { actor_system }
    }

    pub fn from_system(system: &ActorSystem) -> Self {
        let actor_system = system.inner.clone();
        RootContext { actor_system }
    }

    pub fn spawn<A>(&self, props: Props<A>) -> ActorRef<A>
    where
        A: Actor + Send + 'static,
    {
        let mailbox_config = props.mailbox;
        let mailbox = Mailbox::<A>::new(mailbox_config);
        let mut ctx = Context::new(mailbox, self.actor_system.clone());
        let actor = (props.producer)();
        let actor_ref = ctx.actor_ref();
        ctx.initialize(actor);
        actor_ref
    }

    pub fn spawn_named<A>(&self, props: Props<A>, name: String) -> ActorRef<A>
    where
        A: Actor + Sync + Send + 'static,
    {
        let mailbox_config = props.mailbox;
        let mailbox = Mailbox::<A>::new(mailbox_config);
        let mut ctx = Context::new(mailbox, self.actor_system.clone());
        let actor = (props.producer)();
        let actor_ref = ctx.actor_ref();
        ctx.initialize(actor);
        self.actor_system
            .write()
            .unwrap()
            .register_actor(name, actor_ref.clone());
        actor_ref
    }

    pub async fn request_async<M, A>(&self, target: ActorRef<A>, msg: M) -> M::Result
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Actor + Handler<M>,
    {
        // sending meesage that requires a response need to create MessageEnvelope with a oneshot channel
        // then schedule it processing by using the mailbox sender in ActorRef. The mailbox passes the
        // message to the actor's receive method. The actor can then process the message as Handler<M>
        // which will return a result. The result is then sent back to the oneshot channel and the
        // oneshot channel is returned to the caller.
        let (sender, receiver) = oneshot::channel();
        let envelope: MessageEnvelope<A, M> = MessageEnvelope::new(msg, Some(sender));
        target.send_user_message(envelope);
        receiver.await.unwrap()
    }
}
