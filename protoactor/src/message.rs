use std::sync::Arc;

/// Marker trait for messages. The `Result` associated type is used to specify the type of the
/// result that will be returned when the message is sent to an actor.
/// # Example
/// ```
/// use protoactor::message::Message;
///
/// struct MyMessage;
///
/// impl Message for MyMessage {
///    type Result = ();
/// }
/// ```
pub trait Message {
    /// The type of the result that will be returned when the message is sent to an actor.
    type Result: Send + 'static;
}

/// Blanket implementation of `Message` for all `Arc` type so that Message don't have to be
/// re-implemented if sent as `Arc`.
impl<M> Message for Arc<M>
where
    M: Message,
{
    type Result = M::Result;
}

/// Blanket implementation of `Message` for all `Box` type so that Message don't have to be
/// re-implemented if sent as `Box`.
impl<M> Message for Box<M>
where
    M: Message,
{
    type Result = M::Result;
}
