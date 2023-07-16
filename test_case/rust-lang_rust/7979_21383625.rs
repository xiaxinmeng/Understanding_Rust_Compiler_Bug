 rust
fn output_base_name(config: &config, testfile: &Path) -> Path {
    config.build_base
        .push_rel(&output_testname(testfile))
        .with_filetype(config.stage_id)
}

fn aux_output_dir_name(config: &config, testfile: &Path) -> Path {
    output_base_name(config, testfile).with_filetype("libaux")
}

fn make_exe_name(config: &config, testfile: &Path) -> Path {
    Path(output_base_name(config, testfile).to_str() + os::EXE_SUFFIX)
}

#[cfg(target_os = "win32")]
fn target_env(lib_path: &str, prog: &str) -> ~[(~str,~str)] {

    let mut env = os::env();

    // Make sure we include the aux directory in the path
    assert!(prog.ends_with(".exe"));
    let aux_path = prog.slice(0u, prog.len() - 4u).to_owned() + ".libaux";

    env = do env.map() |pair| {
        let (k,v) = (*pair).clone();
        if k == ~"PATH" { (~"PATH", v + ";" + lib_path + ";" + aux_path) }
        else { (k,v) }
    };
    if prog.ends_with("rustc.exe") {
        env.push((~"RUST_THREADS", ~"1"));
    }
    return env;
}
