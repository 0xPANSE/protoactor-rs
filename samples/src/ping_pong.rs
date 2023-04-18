use log::info;
use protoactor::actor::{Actor, Context, Handler};
use protoactor::actor_ref::ActorRef;
use protoactor::derive::Message;

#[derive(Debug, Default)]
pub struct PingActor {
    counter: usize,
}

impl Actor for PingActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Ping;

impl Handler<Ping> for PingActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> usize {
        let self_ = ctx.myself();
        let target = format!("PingActor[{}]", self_.id());
        info!(
            target: &target,
            "PingPongActor received a message: {:?}", &msg
        );
        self.counter += 1;
        self.counter
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Pong(usize, #[obfuscated] String);

pub struct PongActor {
    counter: usize,
    actor_ref: Option<ActorRef<PingActor>>,
}

// impl Actor for PongActor {
//     type Context = Context<Self>;
//
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.actor_ref = Some(ctx.spawn(PingActor::default(), "PingActor"));
//     }
// }
