use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// An enum representing a mailbox that can be either bounded or unbounded.
pub enum Mailbox<M: Send> {
    Bounded(Sender<M>),
    Unbounded(UnboundedSender<M>),
}

impl<M: Send> Mailbox<M> {
    /// Creates an unbounded mailbox and returns a tuple containing the sender and receiver.
    pub fn unbounded() -> (Self, UnboundedReceiver<M>) {
        let (tx, rx) = unbounded_channel();
        (Mailbox::Unbounded(tx), rx)
    }

    /// Creates a bounded mailbox with the given capacity and returns a tuple containing
    /// the sender and receiver.
    pub fn bounded(capacity: usize) -> (Self, Receiver<M>) {
        let (tx, rx) = channel(capacity);
        (Mailbox::Bounded(tx), rx)
    }

    /// Sends a message using the mailbox, either bounded or unbounded.
    pub fn send(&self, msg: M) {
        match self {
            Mailbox::Bounded(tx) => {
                // Ignoring the error for simplicity, but you may want to handle it.
                let _ = tx.try_send(msg);
            }
            Mailbox::Unbounded(tx) => {
                // Ignoring the error for simplicity, but you may want to handle it.
                let _ = tx.send(msg);
            }
        }
    }
}
