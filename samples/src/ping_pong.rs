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

    fn started(&mut self, ctx: &mut Self::Context) {
        let target = format!("PingActor[{}]", ctx.myself().id());
        info!(target: &target, "started");
        // let pong_actor = ctx.spawn(PongActor {
        //     counter: 0,
        //     ping_actor: ctx.myself(),
        // });
        // pong_actor.send(Ping(ctx.myself()));
    }
}

impl Drop for PingActor {
    fn drop(&mut self) {
        info!("PingActor dropped {}", self.counter);
    }
}

#[derive(Message)]
pub struct Ping(#[hidden] ActorRef<PongActor>);

#[derive(Message)]
pub struct Pong(usize, #[obfuscated] String);

impl Handler<Ping> for PingActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        // let target = format!("PingActor[{}]", ctx.myself().id());
        // info!(target: &target, "received a message: {:?}", &msg);
        self.counter += 1;
        msg.0.send(Pong(self.counter, "Pong".to_string()));
    }
}

pub struct PongActor {
    pub counter: usize,
    pub ping_actor: ActorRef<PingActor>,
}

impl Actor for PongActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // let target = format!("PongActor[{}]", ctx.myself().id());
        // info!(target: &target, "started");
        self.ping_actor.send(Ping(ctx.myself().clone()));
    }
}

impl Handler<Pong> for PongActor {
    fn handle(&mut self, msg: Pong, ctx: &mut Context<Self>) {
        // let target = format!("PongActor[{}]", ctx.myself().id());
        self.counter += 1;
        // info!(
        //     target: &target,
        //     "received a message: {:?} #{}", &msg, self.counter
        // );

        self.ping_actor.send(Ping(ctx.myself().clone()));
    }
}

impl Drop for PongActor {
    fn drop(&mut self) {
        info!("dropped PongActor after {} messages", self.counter);
    }
}
