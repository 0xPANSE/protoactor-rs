//! # ProtoActor
//!
//! This is a Rust port of the [ProtoActor](https://github.com/asynkron/protoactor-dotnet) framework.
//!

extern crate core;

#[cfg(feature = "derive")]
pub mod derive {
    pub use protoactor_derive::*;
}

pub mod actor;
pub mod actor_process;
pub mod actor_ref;
pub mod actor_system;
pub mod config;
pub mod mailbox;
pub mod message;
pub mod props;
pub mod proto;
pub mod utils;

/// prelude module
pub mod prelude {
    pub use crate::actor::{Actor, Context, Handler};
    pub use crate::actor_process::ActorProcess;
    pub use crate::actor_ref::ActorRef;
    pub use crate::mailbox::Mailbox;
    pub use crate::message::Message;
    pub use crate::props::Props;
}

#[cfg(feature = "cluster")]
pub use protoactor_cluster as cluster;
#[cfg(feature = "persistence")]
pub use protoactor_persistence as persistence;
#[cfg(feature = "remote")]
pub use protoactor_remote as remote;
