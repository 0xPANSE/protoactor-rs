//! The `ActorSystem` module provides a foundation for building concurrent and distributed systems using actors.
//!
//! An actor system is responsible for managing the lifecycle of actors and provides an execution environment
//! for actor-based applications. It acts as the root of the actor hierarchy and manages message dispatching,
//! scheduling, and supervision. The `ActorSystem` is a fundamental component of the ProtoActor framework.
//!

use crate::actor::{Actor, ActorRef};
use crate::context::{Context, RootContext};
use crate::props::Props;
use std::ops::DerefMut;
use std::sync::{Arc, Weak};
use tokio::sync::Notify;

pub struct ActorSystem {
    inner: Arc<ActorSystemInner>,
    root: RootContext,
}

pub(crate) struct ActorSystemInner {
    shutdown_signal: Arc<Notify>,
}

impl ActorSystemInner {
    pub fn new() -> Self {
        Self {
            shutdown_signal: Arc::new(Notify::new()),
        }
    }

    pub fn shutdown(&self) {
        self.shutdown_signal.notify_waiters();
    }
}

impl ActorSystem {
    pub fn new() -> Self {
        let actor_system_inner = Arc::new(ActorSystemInner::new());
        let root_context = RootContext::new(actor_system_inner.clone());

        Self {
            inner: actor_system_inner,
            root: root_context,
        }
    }

    pub fn root(&self) -> &RootContext {
        &self.root
    }

    /// Shutdown the actor system.
    pub fn shutdown(&self) {
        self.inner.shutdown();
    }

    /// Wait for the actor system to shutdown.
    /// Use it to block the main thread.
    pub async fn wait_for_shutdown(&self) {
        self.inner.shutdown_signal.notified().await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::actor::{Actor, Handler, Message};
    use crate::props::Props;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::{timeout, Duration};

    struct TestActor {
        counter: usize,
    }

    struct Increment;

    impl Message for Increment {
        type Result = ();
    }

    impl Actor for TestActor {
        type Context = Context<Self>;
    }

    impl Handler<Increment> for TestActor {
        fn handle(&mut self, _msg: Increment, _ctx: &mut Self::Context) {
            self.counter += 1;
        }
    }

    #[tokio::test]
    async fn test_actor_system() {
        let system = ActorSystem::new();
        let props = Props::from_producer(|| TestActor { counter: 0 });
        let pid = system.root().spawn(props);

        // system.root().send(pid.clone(), Increment);
        // system.root().send(pid.clone(), Increment);
        // system.root().send(pid.clone(), Increment);
        //
        // // Wait for messages to be processed
        // timeout(Duration::from_millis(100), system.wait_for_shutdown())
        //     .await
        //     .unwrap();
        //
        // let test_actor = pid
        //     .cell
        //     .lock()
        //     .unwrap()
        //     .actor
        //     .as_any()
        //     .downcast_ref::<TestActor>()
        //     .unwrap();
        // assert_eq!(test_actor.counter, 3);
    }
}
