rust
#![feature(try_blocks)]

struct Response {
    bookmarks: String,
    continue_after: String,
}

#[inline(never)]
fn new_string() -> String {
    String::new()
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
            bookmarks: new_string(),
            continue_after: page?,
            ..def()
        }
    }
}

fn format_response_no_try(page: Result<String, String>) -> Result<Response, String> {
    Ok(
        Response {
            bookmarks: new_string(),
            continue_after: page?,
            ..def()
        }
    )
}
