use protoactor::actor::{Actor, Context, Handler};
use protoactor::message::Message;

pub struct PingPongActor {
    counter: usize,
}

impl Actor for PingPongActor {
    type Context = Context<Self>;
}

impl Default for PingPongActor {
    fn default() -> Self {
        PingPongActor { counter: 0 }
    }
}

pub struct Ping;

impl Message for Ping {
    type Result = usize;
}

impl Handler<Ping> for PingPongActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> usize {
        self.counter += 1;
        self.counter
    }
}
