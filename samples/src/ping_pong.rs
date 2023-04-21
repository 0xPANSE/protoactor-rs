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
#[rtype(Pong)]
pub struct Ping;

impl Handler<Ping> for PingActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Pong {
        let self_ = ctx.myself();
        let target = format!("PingActor[{}]", self_.id());
        info!(
            target: &target,
            "PingPongActor received a message: {:?}", &msg
        );
        self.counter += 1;
        self.counter;
        Pong(self.counter, "pong".to_string())
    }
}

#[derive(Message)]
#[rtype(Ping)]
pub struct Pong(usize, #[obfuscated] String);

pub struct PongActor {
    counter: usize,
    ping_actor: Option<ActorRef<PingActor>>,
}

impl Actor for PongActor {
    type Context = Context<Self>;
}

impl Handler<Pong> for PongActor {
    fn handle(&mut self, msg: Pong, ctx: &mut Context<Self>) -> Ping {
        let self_ = ctx.myself();
        let target = format!("PongActor[{}]", self_.id());
        info!(
            target: &target,
            "PingPongActor received a message: {:?}", &msg
        );
        self.counter += 1;
        ctx.response::<PingActor, Ping>(Ping);
        self.counter;
        Ping
    }
}
