rust
#![feature(try_blocks)]

struct Response {
    bookmarks: String,
    continue_after: String,
}

#[inline(never)]
fn def() -> Response {
    Response {
        bookmarks: String::new(),
        continue_after: String::new(),
    }
}

fn format_response(page: Result<String, String>) -> Result<Response, String> {
    try {
        Response {
            continue_after: page?,
            ..def()
        }
    }
}
