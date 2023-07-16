
It appears that on permission managed windows machines (eg. win7) rust generated binaries require administrator privileges to run, even for trivial case applications like println!("Hello World"); these permissions must be accepted ("Run this as administrator") every time the application is launched.

I have no idea how this is typically managed in win programming, but having to accept "run this as administrator" every time you run cargo test or launch a binary is really tedious.
