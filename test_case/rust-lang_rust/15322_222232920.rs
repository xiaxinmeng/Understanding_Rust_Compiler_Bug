
cargo build --target=arm-unknown-linux-gnueabihf --verbose
       Fresh core v0.1.0 (file:///Users/administrator/Projects/my_project/core)
   Compiling my_project v0.1.0 (file:///Users/administrator/Projects/my_project)
     Running `rustc src/main.rs --crate-name my_project --crate-type bin -g --out-dir /Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug --emit=dep-info,link --target arm-unknown-linux-gnueabihf -C linker=arm-linux-gnueabihf-gcc-4.8 -L dependency=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug -L dependency=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug/deps --extern core=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug/deps/libcore-a11fd4aa80d6a663.rlib`
error: could not exec the linker `arm-linux-gnueabihf-gcc-4.8`: No such file or directory (os error 2)
error: Could not compile `my_project`.

Caused by:
  Process didn't exit successfully: `rustc src/main.rs --crate-name my_project --crate-type bin -g --out-dir /Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug --emit=dep-info,link --target arm-unknown-linux-gnueabihf -C linker=arm-linux-gnueabihf-gcc-4.8 -L dependency=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug -L dependency=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug/deps --extern core=/Users/administrator/Projects/my_project/target/arm-unknown-linux-gnueabihf/debug/deps/libcore-a11fd4aa80d6a663.rlib` (exit code: 101)
