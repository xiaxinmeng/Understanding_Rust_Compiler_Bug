
5   |         let non_send = get_non_send();
    |             -------- has type `Box<dyn std::io::Read>` which is not `Send`
6   |         drop(non_send);
7   |         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    |                                                              ^^^^^^ await occurs here, with `non_send` maybe used later
8   |     });
    |     - `non_send` is later dropped here
