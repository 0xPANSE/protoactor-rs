use log::{info, warn};
use std::time::Instant;

#[derive(Debug)]
enum Msg {
    Ping,
    Report,
    Pong,
}

fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_micros()
        .init();
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let (tx1, mut rx1) = tokio::sync::mpsc::unbounded_channel::<Msg>();
            let (tx2, mut rx2) = tokio::sync::mpsc::unbounded_channel::<Msg>();

            let tx11 = tx1.clone();
            let tx21 = tx2.clone();
            let ping = tokio::spawn(async move {
                let mut count = 0;
                let mut timer: Option<Instant> = None;
                loop {
                    match rx1.recv().await {
                        Some(Msg::Ping) => {
                            if let Some(start) = timer {
                                if start.elapsed().as_secs() >= 1 {
                                    info!( target: "ping", "elapsed: {:?}, counted: {:?}",
                                        start.elapsed(),
                                        count
                                    );
                                    timer = None;
                                }
                            }
                            if tx2.is_closed() {
                                info!("tx2 is closed");
                                break;
                            }
                            tx2.send(Msg::Pong).unwrap();
                        }
                        Some(Msg::Report) => {
                            info!("ping: {:?}", count);
                            timer = Some(Instant::now());
                            count = 0;
                        }
                        _ => {
                            warn!(target: "ping", "rx1.recv() returned None");
                            break;
                        }
                    }
                    count += 1;
                }
            });

            let pong = tokio::spawn(async move {
                let mut count = 0;
                let mut started: Option<Instant> = None;
                loop {
                    match rx2.recv().await {
                        Some(Msg::Pong) => {
                            if let Some(start) = started {
                                if start.elapsed().as_secs() >= 1 {
                                    info!( target: "pong",
                                        "elapsed: {:?}, counted: {:?}",
                                        start.elapsed(),
                                        count
                                    );
                                    started = None;
                                }
                            }
                            if tx1.is_closed() {
                                warn!(target: "pong", "tx1 is closed");
                                break;
                            }
                            tx1.send(Msg::Ping).unwrap();
                        }
                        Some(Msg::Report) => {
                            info!("pong: {:?}", count);
                            started = Some(Instant::now());
                            count = 0;
                        }
                        _ => {
                            warn!(target: "pong", "rx2.recv() returned None");
                            break;
                        }
                    }
                    count += 1;
                }
            });
            tx11.send(Msg::Ping).unwrap();
            // warmup
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // reset counters and report in 1 second
            tx11.send(Msg::Report).unwrap();
            tx21.send(Msg::Report).unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            ping.abort();
            pong.abort();
        });
}
