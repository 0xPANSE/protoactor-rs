use crate::actor::Actor;
use crate::mailbox::{Mailbox, MailboxConfig};
use std::any::type_name;

/// The Props struct is used to define the configuration and initial state for creating an actor.
/// It includes details like the actor's constructor and custom mailbox settings.
pub struct Props<A: Actor> {
    /// A boxed closure that produces a new instance of the actor.
    /// This closure will be called when spawning a new actor.
    pub(crate) producer: Box<dyn Fn() -> A + Send>,

    /// Mailbox producers are used to create a new mailbox for each actor.
    pub(crate) mailbox_producer: Box<dyn Fn() -> Mailbox<A>>,

    pub(crate) prefix: String,
}

impl<A: Actor> Props<A> {
    /// Creates a new Props instance from a producer closure.
    /// The producer closure is a function that returns a new instance of the actor.
    pub fn from_producer(producer: impl Fn() -> A + Send + Sync + 'static) -> Self {
        Props {
            producer: Box::new(producer),
            // default mailbox producer is unbounded
            mailbox_producer: Box::new(|| Mailbox::new(MailboxConfig::Unbounded)),
            prefix: Self::default_prefix(),
        }
    }

    /// Sets the mailbox configuration for the actor.
    /// This can be either bounded or unbounded.
    pub fn with_mailbox(mut self, mailbox_config: MailboxConfig) -> Self {
        self.mailbox_producer = Box::new(move || Mailbox::new(mailbox_config.clone()));
        self
    }

    /// Creates a new actor instance using the producer closure.
    pub(crate) fn produce(&self) -> A {
        (self.producer)()
    }

    /// Creates a new mailbox instance using the mailbox producer closure.
    /// This is used when spawning a new actor.
    pub(crate) fn produce_mailbox(&self) -> Mailbox<A> {
        (self.mailbox_producer)()
    }

    /// Returns the default prefix for the actor type. It is the fully qualified domain name compliant name.
    fn default_prefix() -> String {
        let full_type_name = type_name::<A>();
        let fqdn_compliant_name = full_type_name.replace("::", ".");
        fqdn_compliant_name
    }
}
