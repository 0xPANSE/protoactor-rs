# ProtoActor Rust

Rust implementation of ProtoActor framework. It is ported from the original 
[dotnet implementation](https://github.com/AsynkronIT/protoactor-dotnet).

NOTE: This project is Work in progress. Below is a list of things that are 
still missing or need to be improved.

# Todo

## Actor system features:
- [x] `Actor`: The trait that will be implemented by all actors.
- [ ] `Context`: The struct that will provide the context for an actor.
  - [ ] Respond to sender
  - [ ] Stash current message pending restart
  - [ ] Spawn child actor with automatic/prefixed/specific name
  - [ ] Stop/restart/resume children
  - [ ] Set/push/pop actor behaviors (become/unbecome)
  - [ ] Watch/unwatch actors
  - [ ] Receive timeout
- [x] `Props`: The struct that represents the properties of an actor.
  - [x] `from_producer`: Creates a new Props from a producer function.
  - [ ] `with_mailbox`: Creates a new Props with the given mailbox.
  - [ ] `with_supervisor`: Creates a new Props with the given supervisor strategy.
  - [ ] `with_dispatcher`: Creates a new Props with the given dispatcher.
  - [ ] `with_middlewares`: Creates a new Props with the given middlewares.
  - [ ] `spawn`: Spawns a new actor with the given props.
- [ ] `Process`: The enum that represents an actor's process with `Local` and `Remote` variants.
  - [ ] `ActorProcess`: Represents a local actor's process.
  - [ ] `RemoteProcess`: Represents a remote actor's process.
- [ ] `ActorRef`: The struct that is a reference to an actor process.
  - [ ] Holds address (nonhost or remote address) and ID
  - [ ] Send user message
  - [ ] Send system message
  - [ ] Request
  - [ ] ~~Request future~~ <-- async/await (future) should work out of the box.
  - [ ] Stop
  - [x] deref to protobuf PID message.
- [ ] `Mailbox`: The mailbox implementation for the actor system.
- [ ] `ProcessRegistry`: The struct that stores and manages actor processes in the actor system.
  - Get Process by PID
  - Add local Process with ID
  - Remove Process by PID
  - Generate next Process ID
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
struct Pong;

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
    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Pong {
        self.counter += 1;
        println!("Received ping: {}", self.counter);
        Pong
    }
}


fn main() {
    tokio::run(future::lazy(|| {
        let system = ActorSystem::new();
        let props = Props::from_producer(|| MyActor { counter: 0 });
        // using PID as an address
        let pid = system.root().spawn(props);
        system.root().send(pid, Ping);
        // suing ActorRef as an address
        let actor_ref = system.root().spawn(props);
        // wait for ctl-c
        future::empty();
        Ok(())
    }));
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
