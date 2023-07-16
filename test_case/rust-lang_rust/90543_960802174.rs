
$ uname -p
aarch64
$ docker run -it --rm ubuntu
root@9779d2dedc08:/# apt-get update -qq && apt-get install -y curl build-essential > /dev/null 2> /dev/null
root@9779d2dedc08:/# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -q > /dev/null 2>/dev/null
root@9779d2dedc08:/# source $HOME/.cargo/env
root@9779d2dedc08:/# cargo init helloworld
     Created binary (application) package
root@9779d2dedc08:/# cd helloworld/
root@9779d2dedc08:/helloworld# cargo run
   Compiling helloworld v0.1.0 (/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 1.31s
     Running `target/debug/helloworld`
Hello, world!
root@9779d2dedc08:/helloworld# cat > src/main.rs
struct Greeting {
    name: String,
}

fn main() {
    let greeting = Greeting {
        name: "Bjørn".to_string()
    };

    println!("Hello, {}!", greeting.name);
}
root@9779d2dedc08:/helloworld# cargo run
   Compiling helloworld v0.1.0 (/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/helloworld`
Hello, Bjørn!
