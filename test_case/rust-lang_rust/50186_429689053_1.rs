rust
things.update(|v| v.push(4)); // returns ()
// Alternative with v by-move. I'd guess this one isn't as useful, and you can easily use
// mem::swap or mem::replace in the by-ref version instead if you need it.
things.update(|v| { v.push(4); v });
let id = next_id.get_and_update(|x| x + 1);
let now = current_time.update_and_get(|x| x + 50);
