rust
async fn handle(
    guest_client: Client, 
    host_client: &mut Client,
    result: ClientListenResult,
    host: &mut Member<Self>, // This is wrong, Self = Guest
    guest: &mut Member<Guest>
) {}

async fn handle(
    guest_client: Client, 
    host_client: &mut Client,
    result: ClientListenResult,
    host: &mut Member<Host>, // This is the correct type
    guest: &mut Member<Guest>
) {}
