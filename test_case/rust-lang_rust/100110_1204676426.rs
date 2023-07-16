rust
    fn handle_pty_start(&self, value: String) {
        info!("=>handle_pty_start {}", value);
        let pty_start: PtyStart = match serde_json::from_str(&value) {
            Ok(v) => v,
            _ => return,
        };

        let session_id = pty_start.session_id;
        let user_name = if pty_start.user_name.len() == 0 {
            #[cfg(unix)]
            {
                let name = get_current_username().unwrap();<-------------------sometimes here.
                String::from(name.to_str().unwrap())
            }
            #[cfg(windows)]
            get_current_user()
        } else {
            pty_start.user_name
        };

        let mut flag: u32 = 0;
        if pty_start.init_block {
            flag = flag | PTY_FLAG_INIT_BLOCK
        }

        self.running_task_num.fetch_add(1, SeqCst);
        let pty_session =
            match ConPtySystem::default().openpty(&user_name, pty_start.cols, pty_start.rows, flag)
            {
                Ok(session) => session,
                Err(e) => {
                    error!("=>openpty err {}", e.to_string());
                    let msg = self.build_error_msg(session_id.clone(), e);
                    self.event_bus.dispatch(PTY_WS_MSG, msg);
                    self.running_task_num.fetch_sub(1, SeqCst);
                    return;
                }
            };

        let input_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let writer = pty_session.get_writer().unwrap();
        let session = Arc::new(Session {
            session_id: session_id.clone(),
            pty_session,
            writer: Arc::new(Mutex::new(writer)),
            last_input_time: Arc::new(AtomicU64::new(input_time)),
            is_stoped: Arc::new(AtomicBool::new(false)),
        });

        self.session_map
            .write()
            .unwrap()
            .insert(session_id.clone(), session.clone());

        let msg = self.build_ready_msg(session_id.clone());
        self.event_bus.dispatch(PTY_WS_MSG, msg);

        let self_0 = self.clone();
        self.runtime
            .spawn(async move { self_0.report_output(session).await });

        info!("handle_pty_start success");<-------------------sometimes here.
    }
