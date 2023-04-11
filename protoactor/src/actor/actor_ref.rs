use super::actor_process::ActorProcess;
use crate::actor::Message;
use crate::prelude::Actor;
use crate::proto::Pid;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::oneshot;

/// The ActorRef struct is a reference to an actor process.
/// It holds the `Pid` that uniquely identifies the actor process and the `ActorProcess` that handles
/// the actual processing of messages for the actor. The tell method is used to send a message to
/// the actor process. The `Deref` trait is implemented to allow getting the `Pid` from an `ActorRef`.
#[derive(Clone)]
pub struct ActorRef<A: Actor> {
    // The Pid that uniquely identifies the actor process.
    pid: Pid,

    // The ActorProcess that handles the actual processing of messages for the actor.
    process: Arc<ActorProcess>,

    #[doc(hidden)]
    _marker: PhantomData<A>,
}

impl<A: Actor> ActorRef<A> {
    pub(crate) fn new(pid: Pid, process: Arc<ActorProcess>) -> Self {
        ActorRef {
            pid,
            process,
            _marker: PhantomData,
        }
    }

    pub fn send<M>(&self, msg: M, tx: oneshot::Sender<M::Result>)
    where
        M: Message,
    {
    }
}

// Implement the Deref trait to allow getting the Pid from an ActorRef.
// This allows using the `*` operator to get the Pid from an ActorRef and use pid in user
// messages since Pid can be used in Grpc messages that are sent to other actor systems in the
// cluster.
impl<A: Actor> Deref for ActorRef<A> {
    type Target = Pid;

    fn deref(&self) -> &Self::Target {
        &self.pid
    }
}
