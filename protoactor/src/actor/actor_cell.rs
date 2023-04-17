use crate::actor::context::ActorContext;
use crate::actor::{Actor, Context};
use crate::mailbox::Mailbox;
use tokio::sync::oneshot;

pub(crate) struct ActorCell<A>
where
    A: Actor,
{
    actor: A,
    ctx: A::Context,
    mailbox: Mailbox<A>,
    stopping: oneshot::Receiver<()>,
}

impl<A: Actor> ActorCell<A> {
    pub(crate) fn new(mut ctx: A::Context, actor: A, mailbox: Mailbox<A>) -> Self {
        let (stop_sender, stopping) = oneshot::channel();

        ctx.set_stop_sender(stop_sender);

        Self {
            actor,
            ctx,
            mailbox,
            stopping,
        }
    }

    pub(crate) async fn run(mut self) {
        loop {
            tokio::select! {
                envelope = self.mailbox.recv() => {
                    match envelope {
                        Some(mut envelope) => {
                            envelope.handle(&mut self.actor, &mut self.ctx);
                        }
                        None => break,
                    }
                }
                _ = &mut self.stopping => {
                    break;
                }
            }
        }
    }
}