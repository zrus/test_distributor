use bastion::prelude::*;

fn main() {
    Bastion::init();
    let _ = Bastion::supervisor(|sp| {
        sp.with_strategy(SupervisionStrategy::OneForOne)
        // sp
            .children(|children| {
                children
                    .with_distributor(Distributor::named("test"))
                    .with_exec(|ctx| async move {
                        loop {
                            MessageHandler::new(ctx.recv().await?).on_tell(|msg: &str, _| {
                                match msg {
                                    "kill 1" => ctx.supervisor().unwrap().stop().unwrap(),
                                    _ => println!("1: {}", msg),
                                };
                                // println!("1: {}", msg);
                            });
                        }
                    })
            })
    })
    .and_then(|_| {
        Bastion::supervisor(|sp| {
            sp.with_strategy(SupervisionStrategy::OneForOne)
            // sp
                .children(|children| {
                    children
                        .with_distributor(Distributor::named("test"))
                        .with_exec(|ctx| async move {
                            loop {
                                MessageHandler::new(ctx.recv().await?).on_tell(|msg: &str, _| {
                                    match msg {
                                        "kill 2" => ctx.supervisor().unwrap().stop().unwrap(),
                                        _ => println!("2: {}", msg),
                                    };
                                    // println!("2: {}", msg);
                                });
                            }
                        })
                })
        })
    })
    .map_err(|_| println!("can not create bastion"));
    std::thread::sleep(std::time::Duration::from_secs(2));

    Bastion::start();

    for i in 0..100 {
        let test = Distributor::named("test");
        let _ = test.tell_one("hello").expect("can not send message");
        if i == 10 {
            let _ = test.tell_one("kill 1").expect("can not send message");
        }
        if i == 20 {
            let _ = test.tell_one("kill 2").expect("can not send message");
        }
    }

    Bastion::block_until_stopped();
}
