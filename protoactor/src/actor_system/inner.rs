use crate::actor::{Actor, ActorCell, Context};
use crate::actor_ref::ActorRef;
use crate::actor_system::root_context::RootContext;
use crate::config::NO_HOST;
use crate::prelude::Props;
use crate::proto::Pid;
use flurry::HashMap;
use std::any::Any;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub type ProcessRegistry = Arc<HashMap<String, Box<dyn Any + Send + Sync>>>;

pub(crate) struct ActorSystemInner {
    registry: ProcessRegistry,
    next_actor_id: AtomicU64,
}

impl ActorSystemInner {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(HashMap::new()),
            next_actor_id: AtomicU64::new(0),
        }
    }

    pub(crate) fn spawn<A>(&self, props: &Props<A>, root: RootContext) -> ActorRef<A>
    where
        A: Actor<Context = Context<A>>,
    {
        let name = self.next_actor_id.fetch_add(1, Ordering::Relaxed);
        let actor_name = if !props.prefix.is_empty() {
            format!("{}/${}", props.prefix, name)
        } else {
            format!("${}", name)
        };
        self.spawn_named(actor_name, props, root)
    }

    pub(crate) fn spawn_named<A>(
        &self,
        name: String,
        props: &Props<A>,
        root: RootContext,
    ) -> ActorRef<A>
    where
        A: Actor<Context = Context<A>>,
    {
        let registry_pin = self.registry.pin();
        if registry_pin.contains_key(&name) {
            panic!("Actor with name {} already exists", name);
        }
        let actor = props.produce(); // create the actor instance
        let mailbox = props.produce_mailbox(); // create the mailbox
        let mailbox_sender = mailbox.sender();
        let pid = Pid::new(name.clone(), NO_HOST.to_string());
        let actor_ref = ActorRef::new(pid, root, Arc::new(mailbox_sender));
        let ctx = Context::new(actor_ref.clone());
        let actor_cell = ActorCell::new(ctx, actor, mailbox);
        registry_pin.insert(name.clone(), Box::new(actor_ref.clone()));
        drop(registry_pin);

        let reg = self.registry.clone();
        tokio::spawn(async move {
            // todo: supervision
            actor_cell.run().await;
            // Remove the actor from the registry when it stops running.
            let _ = reg.pin().remove(&name);
        });

        actor_ref
    }
}
