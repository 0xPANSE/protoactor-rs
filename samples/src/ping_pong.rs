use log::info;
use protoactor::actor::{Actor, Context, Handler};
use protoactor::actor_ref::ActorRef;
use protoactor::derive::Message;

#[derive(Debug, Default)]
pub struct PingActor {
    counter: usize,
    start: Option<std::time::Instant>,
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
pub struct Sample1Sec;

#[derive(Message)]
pub struct Pong(usize, #[obfuscated] String);

impl Handler<Ping> for PingActor {
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;
        msg.0.send(Pong(self.counter, "Pong".to_string()));
        if let Some(start) = self.start {
            if start.elapsed().as_secs() >= 1 {
                info!(
                    target: ctx.myself().id(),
                    "elapsed: {:?}, counted: {:?}",
                    start.elapsed(),
                    self.counter
                );
                self.start = None;
            }
        }
    }
}

impl Handler<Sample1Sec> for PingActor {
    fn handle(&mut self, _msg: Sample1Sec, _ctx: &mut Context<Self>) {
        self.counter = 0;
        self.start = Some(std::time::Instant::now());
    }
}

pub struct PongActor {
    pub name: String,
    pub counter: usize,
    pub ping_actor: ActorRef<PingActor>,
}

impl Actor for PongActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.name = ctx.myself().id().to_string();
        self.ping_actor.send(Ping(ctx.myself().clone()));
    }
}

impl Handler<Pong> for PongActor {
    fn handle(&mut self, msg: Pong, ctx: &mut Context<Self>) {
        self.counter += 1;
        self.ping_actor.send(Ping(ctx.myself().clone()));
    }
}

impl Drop for PongActor {
    fn drop(&mut self) {
        // info!(target: self.name.as_str() ,"dropped PongActor after {} messages", self.counter);
    }
}
