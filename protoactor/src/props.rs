use crate::actor::Actor;
use std::sync::Arc;

pub struct Props<A: Actor> {
    producer: Arc<dyn Fn() -> A + Send + Sync>,
}

impl<A: Actor> Props<A> {
    pub fn from_producer(producer: impl Fn() -> A + Send + Sync + 'static) -> Self {
        Props {
            producer: Arc::new(producer),
        }
    }
}
