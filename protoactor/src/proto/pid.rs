//! # Pid (Process ID)
//!
//! In the Proto.Actor platform, an actor is referenced by its Process ID, or PID. A PID is a unique process identifier
//! within the actor system, representing an individual actor.
//!
//! Due to Rust's ownership model and typing system, we cannot use a PID directly as a reference. Instead, we introduced
//! the ActorRef type, which serves as a Rust-like reference to the actor. ActorRef is a wrapper around the PID, used
//! for sending messages to the actor.
//!
//! If you need to access the PID of an actor, you can use the dereference operator, like this:
//! ```rust, ignore
//! use protoactor::actor_ref::ActorRef;
//! use protoactor::proto::Pid;
//! // using the dereference function
//! let pid = actor_ref.deref();
//! // using the dereference operator
//! let pid = &*actor_ref;
//! ```
//! A PID consists of an address and an ID. The address is the location of the host where the actor resides. For remote
//! hosts, this could be the IP address of the machine hosting the actor instance. The ID is a unique identifier for the
//! actor on this host, consisting of a unique number and the actor's name in a specific format.
//!
//! The PID serves as a unique identifier for the actor within the actor system. It is used to send messages to the actor
//! and to identify the actor within the system.
//!
//! The PID is a serializable structure that can be sent to another actor or another actor system.

use serde::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Pid {
    /// The address of the actor system that the actor is running in.
    /// For a remote actor, it is the address of the remote actor system where the actor is hosted
    /// In case of single-node actor system, it is the address of the local actor system, and value
    /// is equal to `crate::config::NODE_HOST` ("nohost").
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,

    /// the Id field of a PID is a unique string identifier for an actor instance within an actor system. It is automatically generated when an actor is spawned, and it is used to identify and address the specific actor instance when sending messages.
    ///
    /// Examples of generated Id values might look like:
    /// * "actor-1"
    /// * "user-15"
    /// * "device-7abf34c9"
    ///
    /// These identifiers are usually a combination of a prefix that provides some context about
    /// the actor and a unique value (e.g., a number, UUID, or a combination of both) to
    /// differentiate between different instances of actors.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,

    #[prost(uint32, tag = "3")]
    pub request_id: u32,
}
