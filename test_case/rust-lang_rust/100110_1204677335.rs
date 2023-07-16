rust
    fn openpty(
        &self,
        user_name: &str,
        cols: u16,
        rows: u16,
        #[allow(dead_code)] flag: u32,
    ) -> Result<std::sync::Arc<dyn PtySession + Send + Sync>, String> {
        let user = get_user_by_name(user_name).ok_or(format!("user {} not exist", user_name))?;<-------------------sometimes here.
        let shell_path = cmd_path("bash").ok_or("bash not exist".to_string())?;
        let home_path = user.home_dir().to_str().unwrap_or_else(|| "/tmp");

        let envs = build_envs(user_name, home_path, &shell_path);
        let (mut master, slave) = openpty(user.clone(), cols, rows)?;

        let mut cmd = Command::new("bash");
        unsafe {
            cmd.pre_exec(move || {
                for signo in &[
                    libc::SIGCHLD,
                    libc::SIGINT,
                    libc::SIGQUIT,
                    libc::SIGTERM,
                    libc::SIGALRM,
                ] {
                    libc::signal(*signo, libc::SIG_DFL);
                }

                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGHUP);

                if libc::setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = cmd
            .args(["--login"])
            .uid(0)
            .gid(0)
            .stdin(slave.try_clone().unwrap())
            .stdout(slave.try_clone().unwrap())
            .stderr(slave.try_clone().unwrap())
            .envs(envs)
            .spawn()
            .map_err(|e| format!("spwan err {}", e))?;

        if (flag & PTY_FLAG_INIT_BLOCK) != 0 {
            let init_block = format!("source {};clear\n", BLOCK_INIT);
            let _ = master.write(init_block.as_bytes());
        }

        let pid = child.id() as i32;
        let path_buf = PathBuf::from(format!("{}{}", FD_PATH, slave.as_raw_fd()));
        let path = read_link(path_buf).unwrap_or(PathBuf::from(DEFAULT_DEVICE));
        let path = path.to_str().unwrap_or(DEFAULT_DEVICE);

        let lc = LoginContext::new(pid, path, &user_name, "127.0.0.1");
        lc.login();

        return Ok(Arc::new(UnixPtySession {
            inner: Arc::new(Mutex::new(Inner {
                master,
                slave,
                child,
            })),
        }));
    }
}
