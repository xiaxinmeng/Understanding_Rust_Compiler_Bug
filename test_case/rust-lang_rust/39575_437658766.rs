rust
// cargo-deps: os_pipe
extern crate os_pipe;

mod use_once_pipe {
    //! A rather strange module that creates a pipe and makes sure that
    //! we only ever send a single byte on that pipe.
    use std::io::{Read, Write};
    use os_pipe::{pipe, PipeReader, PipeWriter};

    pub struct Reader(PipeReader);

    impl Reader {
        /// Will block until `write` gets called!
        pub fn read(mut self) -> Option<u8> {
            let mut res = vec![];
            self.0.read_to_end(&mut res).unwrap();
            assert!(res.len() <= 1, "Our contract got violated! Read {:?}.", res);
            res.get(0).map(|b| *b)
        }
    }

    pub struct Writer(PipeWriter);

    impl Writer {
        pub fn write(mut self, byte: u8) {
            self.0.write(&[byte]).unwrap();
        }
    }

    pub fn create() -> (Reader, Writer) {
        let (reader, writer) = pipe().unwrap();
        (Reader(reader), Writer(writer))
    }
}

use std::process::Command;
use std::os::unix::process::CommandExt;
use std::sync::{Mutex, Arc};

fn main() {
    // Small demonstration of use_once_pipe.
    {
        let (r, w) = use_once_pipe::create();
        w.write(42);
        //w.write(23); // not possible because `write` consumes `self`
        assert_eq!(r.read(), Some(42));
    }

    // Now lets break it.
    {

        let (r, w) = use_once_pipe::create();
        let w = Arc::new(Mutex::new(Some(w)));
        let w2 = Arc::clone(&w);

        Command::new("true")
            .before_exec(move || Ok(w2.lock().unwrap().take().unwrap().write(42)))
            .status().unwrap();
        w.lock().unwrap().take().unwrap().write(23);

        r.read();
    }
}
