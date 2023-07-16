
fn forever(body: fn()) -> ! { while true { f(); } core::unreachable(); }
