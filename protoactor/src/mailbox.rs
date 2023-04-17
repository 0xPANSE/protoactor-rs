use crate::actor::{Actor, Handler};
use crate::message::Envelope;
use crate::message::Message;
use tokio::sync::mpsc;

pub struct MailboxSender<A: Actor> {
    unbounded: Option<mpsc::UnboundedSender<Box<dyn Envelope<A>>>>,
    bounded: Option<mpsc::Sender<Box<dyn Envelope<A>>>>,
    mailbox_config: MailboxConfig,
}

impl<A: Actor> MailboxSender<A> {
    pub(crate) fn from_unbounded(sender: mpsc::UnboundedSender<Box<dyn Envelope<A>>>) -> Self {
        Self {
            unbounded: Some(sender),
            bounded: None,
            mailbox_config: MailboxConfig::Unbounded,
        }
    }

    pub(crate) fn from_bounded(sender: mpsc::Sender<Box<dyn Envelope<A>>>) -> Self {
        let capacity = sender.capacity();
        Self {
            unbounded: None,
            bounded: Some(sender),
            mailbox_config: MailboxConfig::Bounded(capacity),
        }
    }

    pub async fn send<M>(
        &self,
        envelope: Box<dyn Envelope<A>>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        M: Message + Send + 'static,
        M::Result: Send + 'static,
        A: Handler<M>,
    {
        match self.mailbox_config {
            MailboxConfig::Bounded(_) => self
                .bounded
                .as_ref()
                .unwrap()
                .send(envelope)
                .await
                .map_err(|e| e.into()),
            MailboxConfig::Unbounded => self
                .unbounded
                .as_ref()
                .unwrap()
                .send(envelope)
                .map_err(|e| e.into()),
        }
    }
}

impl<A: Actor> Clone for MailboxSender<A> {
    fn clone(&self) -> Self {
        Self {
            unbounded: self.unbounded.clone(),
            bounded: self.bounded.clone(),
            mailbox_config: self.mailbox_config.clone(),
        }
    }
}

pub enum Mailbox<A: Actor> {
    Bounded(
        mpsc::Sender<Box<dyn Envelope<A>>>,
        mpsc::Receiver<Box<dyn Envelope<A>>>,
    ),
    Unbounded(
        mpsc::UnboundedSender<Box<dyn Envelope<A>>>,
        mpsc::UnboundedReceiver<Box<dyn Envelope<A>>>,
    ),
}

impl<A: Actor> Mailbox<A> {
    pub fn new(mailbox_config: MailboxConfig) -> Self {
        match mailbox_config {
            MailboxConfig::Bounded(capacity) => {
                let (sender, receiver) = mpsc::channel(capacity);
                Mailbox::Bounded(sender, receiver)
            }
            MailboxConfig::Unbounded => {
                let (sender, receiver) = mpsc::unbounded_channel();
                Mailbox::Unbounded(sender, receiver)
            }
        }
    }

    pub fn sender(&self) -> MailboxSender<A> {
        match self {
            Mailbox::Bounded(sender, _) => MailboxSender::from_bounded(sender.clone()),
            Mailbox::Unbounded(sender, _) => MailboxSender::from_unbounded(sender.clone()),
        }
    }

    pub async fn recv(&mut self) -> Option<Box<dyn Envelope<A>>> {
        match self {
            Mailbox::Bounded(_, receiver) => receiver.recv().await,
            Mailbox::Unbounded(_, receiver) => receiver.recv().await,
        }
    }
}

// `MailboxConfig` enum allows you to choose between a bounded and unbounded mailbox for an
// actor. A bounded mailbox has a specified capacity and will drop messages if it becomes full,
// whereas an unbounded mailbox will never drop messages.
//
// The default configuration is unbounded.
#[derive(Default, Clone, Debug)]
pub enum MailboxConfig {
    /// A bounded mailbox will drop messages if the mailbox is full.
    Bounded(usize),
    /// An unbounded mailbox will never drop messages.
    #[default]
    Unbounded,
}
