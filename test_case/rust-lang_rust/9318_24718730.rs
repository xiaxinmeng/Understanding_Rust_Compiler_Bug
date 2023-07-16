
let handler = io::signal::Handler::new(SIGINT, || {
  println("look, I got a sigint!");
});
handler.install();
// handler is un-registered when it is destroyed.
