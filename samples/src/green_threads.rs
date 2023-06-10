#[derive(Debug)]
enum Msg {
    Ping,
    Report,
    Pong,
}

fn main() {
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
                loop {
                    match rx1.recv().await {
                        Some(Msg::Ping) => {
                            tx2.send(Msg::Pong).unwrap();
                        }
                        Some(Msg::Report) => {
                            println!("Ping count: {}", count);
                            break;
                        }
                        _ => {
                            println!("rx1.recv() returned None");
                            break;
                        }
                    }
                    count += 1;
                    tx2.send(Msg::Pong).unwrap();
                }
            });

            let pong = tokio::spawn(async move {
                let mut count = 0;
                loop {
                    match rx2.recv().await {
                        Some(Msg::Pong) => {
                            tx1.send(Msg::Ping).unwrap();
                        }
                        Some(Msg::Report) => {
                            println!("Ping count: {}", count);
                            break;
                        }
                        _ => {
                            println!("rx2.recv() returned None");
                            break;
                        }
                    }
                    count += 1;
                    tx1.send(Msg::Ping).unwrap();
                }
            });
            tx11.send(Msg::Ping).unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            tx21.send(Msg::Report).unwrap();
            tx11.send(Msg::Report).unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let j = tokio::join!(ping, pong);
            println!("j: {:?}", j);
        });
}
