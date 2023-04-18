//! The `ActorSystem` module provides a foundation for building concurrent and distributed systems using actors.
//!
//! An actor system is responsible for managing the lifecycle of actors and provides an execution environment
//! for actor-based applications. It acts as the root of the actor hierarchy and manages message dispatching,
//! scheduling, and supervision. The `ActorSystem` is a fundamental component of the ProtoActor framework.
//!

pub(crate) mod inner;
pub(crate) mod root_context;

use crate::config::ActorSystemConfig;
use inner::ActorSystemInner;
use root_context::RootContext;
use std::sync::Arc;

pub struct ActorSystem {
    pub(crate) inner: Arc<ActorSystemInner>,
    pub(crate) config: Arc<ActorSystemConfig>,
}

impl ActorSystem {
    pub fn new(config: ActorSystemConfig) -> Self {
        let inner = Arc::new(ActorSystemInner::new());
        let config = Arc::new(config);
        ActorSystem { inner, config }
    }

    pub fn root(&self) -> RootContext {
        RootContext::new(self.inner.clone())
    }

    pub fn config(&self) -> Arc<ActorSystemConfig> {
        self.config.clone()
    }
}
