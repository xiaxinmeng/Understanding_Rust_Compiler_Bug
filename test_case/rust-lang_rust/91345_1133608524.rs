rust
struct Cookie {
    size: usize,
};

let maybe_cookie = Some(Cookie { size: 5 });

maybe_cookie.inspect(|mut c| c.size = 3);
