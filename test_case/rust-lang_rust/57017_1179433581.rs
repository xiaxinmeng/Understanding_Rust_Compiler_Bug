rs
{ let temporary = &client;
    match temporary.status() { 200 => {
        …
    }}
} // <- temporary dropped here
