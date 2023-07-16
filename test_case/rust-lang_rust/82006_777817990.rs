sh
gdb -ex 'info func' -ex 'quit' ~/rpmbuild/BUILD/foo-1.0.0/target/release/foo | head -n 20
