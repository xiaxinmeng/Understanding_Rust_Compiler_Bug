 rust
        if buf.len() > dst.len() {
            return Err(std::io::IoError { kind: std::io::OtherIoError, desc: "", detail: None });     
   }
