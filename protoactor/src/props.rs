use crate::actor::Actor;
use crate::mailbox::{Mailbox, MailboxConfig};

/// The Props struct is used to define the configuration and initial state for creating an actor.
/// It includes details like the actor's constructor and custom mailbox settings.
pub struct Props<A: Actor> {
    /// A boxed closure that produces a new instance of the actor.
    /// This closure will be called when spawning a new actor.
    pub(crate) producer: Box<dyn Fn() -> A + Send>,
    /// The mailbox configuration for the actor.
    /// This can be either bounded or unbounded.
    pub(crate) mailbox: MailboxConfig,

    /// Mailbox producers are used to create a new mailbox for each actor.
    pub(crate) mailbox_producer: Option<Box<dyn Fn() -> Box<Mailbox<A>> + Send>>,
}

impl<A: Actor> Props<A> {
    /// Creates a new Props instance from a producer closure.
    /// The producer closure is a function that returns a new instance of the actor.
    pub fn from_producer<F>(f: F) -> Self
    where
        F: Fn() -> A + Send + 'static,
    {
        Props {
            producer: Box::new(f),
            mailbox: MailboxConfig::default(),
            // default mailbox producer is unbounded
            mailbox_producer: Some(Box::new(|| {
                Box::new(Mailbox::new(MailboxConfig::Unbounded))
            })),
        }
    }

    /// Sets the mailbox configuration for the actor.
    /// This can be either bounded or unbounded.
    pub fn with_mailbox(mut self, mailbox: MailboxConfig) -> Self {
        self.mailbox = mailbox;
        self
    }
}
