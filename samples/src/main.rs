mod ping_pong;

use crate::ping_pong::{PingActor, PongActor};
use log::info;
use protoactor::actor_system::ActorSystem;
use protoactor::config::ActorSystemConfig;
use protoactor::props::Props;

// single threaded tokio runtime
// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
// #[tokio::main(flavor = "current_thread")]
fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_micros()
        .init();
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .global_queue_interval(1)
        .event_interval(1)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let config = ActorSystemConfig::builder()
                .with_name("ping-pong-actor-system")
                .build();
            let actor_system = ActorSystem::new(config);
            let root_context = actor_system.root();

            let ping_props = Props::<PingActor>::from_producer(Default::default);
            let ping_actor = root_context.spawn(&ping_props);
            let ping_actor2 = ping_actor.clone();
            let pong_props = Props::<PongActor>::from_producer(move || PongActor {
                name: "PongActor".to_string(),
                counter: 0,
                ping_actor: ping_actor2.clone(),
            });

            for i in 0..1 {
                let _pong_actor =
                    root_context.spawn_named(format!("pong/{}", i).as_str(), &pong_props);
            }
            ping_actor.send(ping_pong::Sample1Sec);

            info!("Waiting for 1 second on main thread...");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            info!("Done sleeping on main thread.");
        });
}
