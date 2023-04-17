use log::info;
use protoactor::actor::{Actor, Context, Handler};
use protoactor::derive::Message;
use std::thread::sleep;

#[derive(Debug, Default)]
pub struct PingPongActor {
    counter: usize,
}

impl Actor for PingPongActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Ping;

impl Handler<Ping> for PingPongActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> usize {
        let self_ = ctx.self_();
        let target = format!("PingPongActor[{}]", self_.id());
        info!(
            target: &target,
            "PingPongActor received a message: {:?}", &msg
        );
        self.counter += 1;
        self.counter
    }
}
