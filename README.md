# ProtoActor Rust

Rust implementation of ProtoActor framework. It is ported from the original 
[dotnet implementation](https://github.com/AsynkronIT/protoactor-dotnet).

NOTE: This project is Work in progress. Below is a list of things that are 
still missing or need to be improved.

# Todo

## Simplest possible version:
- [x] `Actor`: The trait that will be implemented by all actors.
- [x] `Context`: The struct that will provide the context for an actor.
- [x] `Pid`: The struct that represents a unique process ID.
- [x] `ActorProcess`: The trait that represents an actor's process.
- [x] `LocalActorProcess`: The struct that represents a local actor's process.
- [x] `ActorRef`: The struct that is a reference to an actor process.
- [x] `Mailbox`: The mailbox implementation for the actor system.
- [x] `ProcessRegistry`: The struct that stores and manages actor processes in the actor system.
- [ ] `Props`: The struct that represents the properties of an actor.
- [ ] `ActorSystem`: The struct that represents an actor system.
- [ ] `RootContext`: The struct that represents the root context of an actor system.
- [ ] `RootActorProcess`: The struct that represents the root actor process of an actor system.

Above should be enough to implement a simple actor system that can spawn actors and send messages to them.

```rust
use protoactor::{Actor, ActorSystem, Context, Message, Props};

struct MyActor {
    counter: usize,
}

struct Ping;

impl Message for Ping {
    type Result = Pong;
}

impl Actor for MyActor {
    type Context = Context<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started");
    }
}

impl HandleMessage<Ping> for MyActor {
    type Result = Pong;
    
    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) {
        self.counter += 1;
        println!("Received ping: {}", self.counter);
    }
}



struct Pong;

fn main() {
    let system = ActorSystem::new();
    let props = Props::from_producer(|| MyActor { counter: 0 });
    let pid = system.root().spawn(props);
    system.root().send(pid, Ping);
    // wait for ctrl-c
    system.wait_for_shutdown();
}
```


## Improved version of the actor system:
- [ ] `ActorSystem`: The struct that represents an actor system.
- [ ] `ActorSystemBuilder`: The struct that builds an actor system.
- [ ] `ActorSystemConfig`: The struct that represents the configuration of an actor system.
- [ ] `ActorSystemConfigBuilder`: The struct that builds an actor system configuration.
- [ ] `ActorSystemEvent`: The enum that represents an event that can occur in an actor system.
- [ ] `ActorSystemEventStream`: The struct that represents the event stream of an actor system.
- [ ] `ActorSystemEventStreamBuilder`: The struct that builds an actor system event stream.
- [ ] `ActorSystemEventStreamConfig`: The struct that represents the configuration of an actor system event stream.
- [ ] `DeadLetter`: The struct that represents a dead letter.
- [ ] `DeadLetterThrottler`: The struct that represents a dead letter throttler.
- [ ] `Middleware`: The trait that represents a middleware.
- [ ] `MiddlewareChain`: The struct that represents a middleware chain.
