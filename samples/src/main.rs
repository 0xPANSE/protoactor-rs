mod ping_pong;

use crate::ping_pong::{Ping, PingPongActor};
use protoactor::actor_system::ActorSystem;
use protoactor::config::ActorSystemConfig;
use protoactor::props::Props;

#[tokio::main]
async fn main() {
    let config = ActorSystemConfig::builder()
        .with_name("ping-pong-actor-system")
        .build();
    let actor_system = ActorSystem::new(config);
    let root_context = actor_system.root();

    let props = Props::<PingPongActor>::from_producer(|| Default::default());
    let ping_pong_actor = root_context.spawn(props);

    let result: usize = root_context.request_async(ping_pong_actor, Ping).await;

    // wait for 1 second using tokio::time::delay_for then exit
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("Received response: {}", result);
}
