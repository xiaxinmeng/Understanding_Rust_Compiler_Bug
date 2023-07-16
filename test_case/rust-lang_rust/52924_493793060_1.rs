rust
let x = async {
    let everything = async {
        let (ctrl, starter) = connection_builder.build().unwrap();
        dbg!(std::mem::size_of_val(&starter));
        let conn_fut = async move {
            let conn_res = starter.start_connection().await;
            trace!("Res: {:?}", conn_res);
        };

        let send_task_futs = join_all((0..8).map(|_| send_msgs(&ctrl)));
        let lifecycle_fut = lifecycle_task(&ctrl);

        dbg!(std::mem::size_of_val(&conn_fut));
        dbg!(std::mem::size_of_val(&send_task_futs));
        dbg!(std::mem::size_of_val(&lifecycle_fut));

        let joiner = Joiner {
                a: Some(conn_fut),
                b: Some(send_task_futs),
                c: Some(lifecycle_fut),
                a_res: None,
                b_res: None,
                c_res: None,
        };

        dbg!(std::mem::size_of_val(&joiner));
        joiner.await;
    };
    dbg!(std::mem::size_of_val(&everything));
    everything.await
};

dbg!(std::mem::size_of_val(&x));

block_on(x);
