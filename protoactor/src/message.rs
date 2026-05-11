use crate::actor::{Actor, Handler};
use crate::actor_ref::SenderRef;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

/// Marker trait for messages. The `Result` associated type is used to specify the type of the
/// result that will be returned when the message is sent to an actor.
/// # Example
/// ```
/// use protoactor::message::Message;
///
/// #[derive(Debug)]
/// struct MyMessage;
///
/// impl Message for MyMessage {}
/// ```
pub trait Message: Debug {}

/// Blanket implementation of `Message` for all `Arc` type so that Message don't have to be
/// re-implemented if sent as `Arc`.
impl<M> Message for Arc<M> where M: Message {}

/// Blanket implementation of `Message` for all `Box` type so that Message don't have to be
/// re-implemented if sent as `Box`.
impl<M> Message for Box<M> where M: Message {}

/// The `Envelope` trait is a wrapper around an actor message that allows you
/// to store messages of different types in a single mailbox. It is also
/// responsible for invoking the appropriate handler for the wrapped message
/// when it's being processed by the actor.
pub trait Envelope<A>
where
    A: Actor,
    Self: Send + 'static,
{
    /// This method handles the wrapped message by calling the appropriate
    /// handler on the given actor with the provided context.
    fn handle(&mut self, actor: &mut A, ctx: &mut A::Context);

    fn as_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<A: Actor> Debug for dyn Envelope<A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_debug(f)
    }
}

/// The `MessageEnvelope` struct is a concrete implementation of the `Envelope`
/// trait for messages implementing the `Message` trait. It stores the message
/// and uses the `Handler` trait to invoke the appropriate handler on the actor
/// when the `handle` method is called.
pub struct MessageEnvelope<A, M>
where
    A: Actor,
    M: Message + Send + 'static,
{
    sender: Option<SenderRef>,
    message: Option<M>,
    _marker: PhantomData<A>,
}

impl<A, M> MessageEnvelope<A, M>
where
    A: Actor,
    M: Message + Send + 'static,
{
    /// Constructs a new `MessageEnvelope` with the given message.
    pub fn new(message: M, sender: Option<SenderRef>) -> Self {
        MessageEnvelope {
            sender,
            message: Some(message),
            _marker: PhantomData,
        }
    }
}

/// The implementation of the `Envelope` trait for `MessageEnvelope`.
/// It requires that the actor implementing the `Handler` trait for the given message type.
impl<A, M> Envelope<A> for MessageEnvelope<A, M>
where
    M: Message + Send + 'static,
    A: Actor + Send + Handler<M>,
{
    /// The `handle` method implementation for `MessageEnvelope`.
    /// It calls the appropriate handler on the actor for the wrapped message.
    #[inline]
    fn handle(&mut self, actor: &mut A, ctx: &mut A::Context) {
        if let Some(msg) = self.message.take() {
            actor.handle(msg, ctx);
        }
    }

    fn as_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageEnvelope")
            .field("actor", &std::any::type_name::<A>())
            .field("message", &std::any::type_name::<M>())
            .finish()
    }
}
