//! # ProtoActor
//!
//! This is a Rust port of the [ProtoActor](https://github.com/asynkron/protoactor-dotnet) framework.
//!

pub mod actor;
pub mod actor_system;
pub mod config;
pub mod context;
pub mod mailbox;
pub mod props;
pub mod proto;

/// prelude module
pub mod prelude {
    pub use crate::actor::{Actor, ActorProcess, ActorRef, Message};
    pub use crate::actor_system::ActorSystem;
    pub use crate::config::ActorSystemConfig;
    pub use crate::context::Context;
}

#[cfg(feature = "cluster")]
pub use protoactor_cluster as cluster;
#[cfg(feature = "persistence")]
pub use protoactor_persistence as persistence;
#[cfg(feature = "remote")]
pub use protoactor_remote as remote;
