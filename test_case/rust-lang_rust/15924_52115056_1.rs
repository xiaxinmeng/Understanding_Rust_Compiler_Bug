
          request.respond (tiny_http::Response::new (
            tiny_http::StatusCode (500),
            std::vec::Vec::new(),  // Vec<Header>
            std::io::BufReader::new ("Rendering error.".as_bytes()),
            None, None
          ));
