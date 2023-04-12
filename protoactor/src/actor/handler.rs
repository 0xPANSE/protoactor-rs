use crate::actor::Actor;
use crate::message::Message;

// The Handler trait defines the interface for handling messages of a specific
// type. It is implemented by actors that can process messages of that type.
/// # Example
/// ```
/// use protoactor::actor::{Actor, Handler};
/// use protoactor::message::Message;
///
/// struct MyMessage;
///
/// impl Message for MyMessage {
///     type Result = ();
/// }
///
/// struct MyActor;
///
/// impl Actor for MyActor {
///     type Context = ();
/// }
///
/// impl Handler<MyMessage> for MyActor {
///     fn handle(&mut self, msg: MyMessage, ctx: &mut Self::Context) -> () {
///         println!("MyActor received a message");
///     }
/// }
///
/// ```
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
    M::Result: Send + 'static,
{
    /// The method called to handle a specific message type.
    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> M::Result;

    // todo: #[async_trait], and check the cost of it
}
