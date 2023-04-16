use crate::actor::Actor;
use crate::actor_process::ActorProcess;
use crate::actor_ref::ActorRef;
use crate::prelude::Props;
use flurry::HashMap;
use std::any::Any;

pub type ProcessRegistry = HashMap<String, Box<dyn Any + Send + Sync>>;

pub(crate) struct ActorSystemInner {
    process_registry: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl ActorSystemInner {
    pub fn registry(&self) -> &ProcessRegistry {
        &self.process_registry
    }
}

impl ActorSystemInner {
    pub fn new() -> Self {
        Self {
            process_registry: HashMap::new(),
        }
    }

    pub fn register_actor<A>(&self, id: String, actor_ref: ActorRef<A>)
    where
        A: Actor + Sync,
    {
        self.process_registry
            .pin()
            .insert(id, Box::new(actor_ref) as Box<dyn Any + Send + Sync>);
    }

    pub fn get_actor_ref<A>(&self, id: &str) -> Option<ActorRef<A>>
    where
        A: Actor + Sync,
    {
        self.process_registry.pin().get(id).and_then(|actor_ref| {
            actor_ref
                .downcast_ref::<ActorRef<A>>()
                .map(|actor_ref| actor_ref.clone())
        })
    }

    fn create_actor<A>(&self, props: Props<A>) -> (ActorRef<A>, ActorProcess<A>)
    where
        A: Actor + 'static,
    {
        todo!("Implement ActorSystemInner::create_actor")
    }

    fn spawn_actor<A>(&self, actor_process: ActorProcess<A>)
    where
        A: Actor + 'static,
    {
        todo!("Implement ActorSystemInner::spawn_actor")
    }
}
