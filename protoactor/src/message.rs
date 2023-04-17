use crate::actor::{Actor, Handler};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::oneshot;

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
pub trait Message {
    /// The type of the result that will be returned when the message is sent to an actor.
    type Result: 'static;
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

pub struct MessageResult<M>(M::Result)
where
    M: Message;

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
    M::Result: Send + 'static,
{
    sender: Option<oneshot::Sender<M::Result>>,
    message: Option<M>,
    _marker: PhantomData<A>,
}

impl<A, M> MessageEnvelope<A, M>
where
    A: Actor,
    M: Message + Send + 'static,
    M::Result: Send + 'static,
{
    /// Constructs a new `MessageEnvelope` with the given message.
    pub fn new(message: M, sender: Option<oneshot::Sender<M::Result>>) -> Self {
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
    M::Result: Send + 'static,
    A: Actor + Send + Handler<M>,
{
    /// The `handle` method implementation for `MessageEnvelope`.
    /// It calls the appropriate handler on the actor for the wrapped message.
    fn handle(&mut self, actor: &mut A, ctx: &mut A::Context) {
        if let Some(msg) = self.message.take() {
            let result = actor.handle(msg, ctx);
            if let Some(sender) = self.sender.take() {
                let _ = sender.send(result);
            }
        }
    }

    fn as_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageEnvelope")
            .field("actor", &std::any::type_name::<A>())
            .field("message", &std::any::type_name::<M>())
            .finish()
    }
}

/// Macro rule that implements protoactor::message::Message for a given type. It accepts return type
/// as argument. It also include field attributes `#[obfuscated]` to hide the field in logs if value
/// is not None in case Debug is not implemented for the type.
/// # Example
/// ```
/// // you need to enable the `derive` feature to use this macro
/// use protoactor::derive::Message;
///
/// #[derive(Message)]
/// struct MyMessage {
///     #[obfuscated]
///     obfuscated: Option<String>,
///     #[hidden]
///     invisible: String,
/// }
///
/// #[derive(Message)]
/// enum MyEnumMessage {
///     Obfuscated(#[obfuscated] Option<String>),
///     NotObfuscated(String),
///     Another{ name: String, #[obfuscated] secret: Option<String> },
/// }
///
/// let my_message = MyMessage {
///     obfuscated: Some("secret".to_string()),
///     invisible: "not visible in logs".to_string()
/// };
///
/// assert_eq!("MyMessage { obfuscated: \"<obfuscated>\" }", format!("{:?}", my_message));
/// ```
#[macro_export]
macro_rules! impl_message {
    ($type:ty, $result:ty) => {
        impl Message for $type {
            type Result = $result;

            fn as_debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 f.debug_struct(stringify!($type))
                    $(.field(stringify!($field), &match self.$field {
                        // if non primitive type and does not implement Message, use Debug
                        #[allow(unused_parens)]
                        value if !value.is_primitive() && !value.is_message() => format!("{:?}", value),
                        #[allow(unused_parens)]
                        value if value.is_message() => format!("{:?}", value),
                        #[allow(unused_parens)]
                        value if value.is_primitive() => value,
                        #[obfuscated] _ => "[RETRACTED]".to_owned(),
                        value => value,
                    }))*
                    .finish()
            }
        }
    };
}
