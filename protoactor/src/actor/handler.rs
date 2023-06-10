use crate::actor::Actor;
use crate::message::Message;

// The Handler trait defines the interface for handling messages of a specific
// type. It is implemented by actors that can process messages of that type.
/// # Example
/// ```
/// use protoactor::actor::{Actor, Context, Handler};
/// use protoactor::message::Message;
///
/// #[derive(Debug)]
/// struct MyMessage;
///
/// impl Message for MyMessage { }
///
/// struct MyActor;
///
/// impl Actor for MyActor {
///     type Context = Context<Self>;
/// }
///
/// impl Handler<MyMessage> for MyActor {
///     fn handle(&mut self, msg: MyMessage, ctx: &mut Self::Context) {
///         println!("MyActor received a message");
///     }
/// }
///
/// ```
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
{
    /// The method called to handle a specific message type.
    fn handle(&mut self, msg: M, ctx: &mut Self::Context);

    // todo: #[async_trait], and check the cost of it
}
