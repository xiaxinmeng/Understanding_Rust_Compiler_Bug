rs
        let orig = manager
            .get_focused()
            .await
            .map(|w| (w, manager.get_framed_window(w)));
