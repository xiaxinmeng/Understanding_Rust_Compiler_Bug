 rust
    if !output.stderr.is_empty() {
        println!("stderr: {:?}", String::from_utf8(output.stderr.clone()));
    }
    assert!(output.stderr.is_empty());
    assert!(output.stdout.is_empty());
    assert!(output.status.success());
