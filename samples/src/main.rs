mod ping_pong;

use crate::ping_pong::{Ping, PingActor};
use log::info;
use protoactor::actor_system::ActorSystem;
use protoactor::config::ActorSystemConfig;
use protoactor::props::Props;

// single threaded tokio runtime
#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();

    let config = ActorSystemConfig::builder()
        .with_name("ping-pong-actor-system")
        .build();
    let actor_system = ActorSystem::new(config);
    let root_context = actor_system.root();

    let props = Props::<PingActor>::from_producer(Default::default);
    let actor1 = root_context.spawn(&props);

    root_context.request_async(&actor1, Ping).await;
    root_context.request_async(&actor1, Ping).await;
    root_context.request_async(&actor1, Ping).await;

    let actor2 = root_context.spawn_named("ping-pong/2", &props);
    let root_context2 = root_context.clone();
    tokio::task::spawn(async move {
        for _ in 0..10 {
            let _ = root_context2.request_async(&actor1, Ping).await;
        }
        info!("From task last Pong");
    });

    info!("Waiting for 1 second on main thread...");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("Done sleeping on main thread.");

    let result = root_context.request_async(&actor2, Ping).await;
    info!("Pong: {:?}", result);
}
