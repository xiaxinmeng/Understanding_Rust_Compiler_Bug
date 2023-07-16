rust
// `host_client` is missing
Guest::handle(guest_client, result, &mut self.host, &mut self.guest).await;

// Fix
Guest::handle(guest_client, &mut host_client, result, &mut self.host, &mut self.guest).await;
