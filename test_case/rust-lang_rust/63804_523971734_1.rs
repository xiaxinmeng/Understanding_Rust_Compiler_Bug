rust
// server connects and is told counter starts at 4
server calls arm(TokenStream(4), TokenStream(5))
  TokenStream::new() returns TokenStream(6)
  TokenStream(6) ends up in TLS (leaking it)
  TokenStream(5) is returned
// server still knows about TokenStream(6)
// server disconnects (taking TokenStream(6)'s actual data with it)
