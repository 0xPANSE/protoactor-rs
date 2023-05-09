mod ping_pong;

use crate::ping_pong::{PingActor, PongActor};
use log::info;
use protoactor::actor_system::ActorSystem;
use protoactor::config::ActorSystemConfig;
use protoactor::props::Props;

// single threaded tokio runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
// #[tokio::main(flavor = "current_thread")]
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

    let ping_props = Props::<PingActor>::from_producer(Default::default);
    let ping_actor = root_context.spawn(&ping_props);
    let ping_actor2 = ping_actor.clone();
    let pong_props = Props::<PongActor>::from_producer(move || PongActor {
        counter: 0,
        ping_actor: ping_actor2.clone(),
    });

    let _pong_actor = root_context.spawn_named("pong/1", &pong_props);

    info!("Waiting for 1 second on main thread...");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("Done sleeping on main thread.");
}
