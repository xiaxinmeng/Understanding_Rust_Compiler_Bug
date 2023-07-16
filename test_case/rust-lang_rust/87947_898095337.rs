
#[test]
fn crash() {
    let config = ResolvedOxidizedPythonInterpreterConfig {
        inner: OxidizedPythonInterpreterConfig {
            argv: Some(vec![std::ffi::OsString::default()]),
            ..OxidizedPythonInterpreterConfig::default()
        },
    };

    // These 2 functions do the same thing. However, only the 2nd crashes.
    //let mut py_config = try_crash(config).unwrap();
    let py_config: pyffi::PyConfig = (&config).try_into().unwrap();
}
