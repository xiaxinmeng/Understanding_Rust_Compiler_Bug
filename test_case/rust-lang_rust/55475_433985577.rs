

   Compiling jpeg-decoder v0.1.15
   Compiling tokio-tcp v0.1.2
   Compiling tokio-uds v0.2.3
   Compiling tokio-udp v0.1.2
   Compiling crossbeam v0.4.1
   Compiling syntex_pos v0.59.1
   Compiling serde_urlencoded v0.5.3
   Compiling serde_json v1.0.32
   Compiling uuid v0.7.1
   Compiling toml v0.4.8
   Compiling csv v1.0.2
   Compiling image v0.20.0
   Compiling tokio v0.1.11
   Compiling syntex_errors v0.59.1
   Compiling hyper v0.12.13
thread 'main' panicked at 'librustc/hir/map/hir_id_validator.rs:31: 
HirIdValidator: The recorded owner of path segment super (id=36924) is ::server[0]::conn[0]::{{?}}[34] instead of ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]
HirIdValidator: Same HirId ::server[0]::conn[0]::{{?}}[34]::{{?}}[0]/2 assigned for nodes path segment super (id=36924) and path segment spawn_all (id=89958)', librustc/util/bug.rs:47:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.
