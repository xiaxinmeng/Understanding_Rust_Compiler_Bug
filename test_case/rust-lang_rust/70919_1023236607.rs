
        // self.conn is Option<Arc<RwLock<_>>>
        let the_ref = self.conn.as_ref().unwrap();
        let mut wg = match the_ref.write() {
            Ok(wg) => wg,
            Err(e) => {
                drop(the_ref);
                drop(e);

                self.conn = None;
                return Err(anyhow::anyhow!("conn lock is poisoned!"));
            }
        };
