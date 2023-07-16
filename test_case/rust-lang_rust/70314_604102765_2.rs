sh
cd refinery && cargo test --features tokio,mysql_async --test mysql_async -- --test-threads 1
