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

use crate::actor_process::ActorProcess;
use bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use prost::DecodeError;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, ::prost::Message)]
pub struct Pid {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub request_id: u32,

    pub process: Option<Arc<ActorProcess<dyn Any>>>,
}

impl Clone for Pid {
    #[inline]
    fn clone(&self) -> Pid {
        Pid {
            address: Clone::clone(&self.address),
            id: Clone::clone(&self.id),
            request_id: Clone::clone(&self.request_id),
            process: Clone::clone(&self.process),
        }
    }
}

impl PartialEq for Pid {
    #[inline]
    fn eq(&self, other: &Pid) -> bool {
        self.address == other.address && self.id == other.id && self.request_id == other.request_id
    }
}

impl ::prost::Message for Pid {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.address != "" {
            ::prost::encoding::string::encode(1u32, &self.address, buf);
        }
        if self.id != "" {
            ::prost::encoding::string::encode(2u32, &self.id, buf);
        }
        if self.request_id != 0u32 {
            ::prost::encoding::uint32::encode(3u32, &self.request_id, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Pid";
        match tag {
            1u32 => {
                let mut value = &mut self.address;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "address");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.id;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "id");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.request_id;
                ::prost::encoding::uint32::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "request_id");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.address != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.address)
        } else {
            0
        } + if self.id != "" {
            ::prost::encoding::string::encoded_len(2u32, &self.id)
        } else {
            0
        } + if self.request_id != 0u32 {
            ::prost::encoding::uint32::encoded_len(3u32, &self.request_id)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.address.clear();
        self.id.clear();
        self.request_id = 0u32;
        self.process = None;
    }
}

impl Default for Pid {
    fn default() -> Self {
        Pid {
            address: ::prost::alloc::string::String::new(),
            id: ::prost::alloc::string::String::new(),
            request_id: 0u32,
            process: None,
        }
    }
}

impl Debug for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Pid")
            .field("address", self.address)
            .field("id", self.id)
            .field("request_id", self.request_id)
            .finish()
    }
}

#[automatically_derived]
impl ::std::hash::Hash for Pid {
    #[inline]
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.id.hash(state);
        self.request_id.hash(state);
    }
}
