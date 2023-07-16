 rust
            let dirpath = match *odir {
                Some(ref d) => d.clone(),
                None => match *input {
                    StrInput(_) => os::getcwd(),
                    FileInput(ref ifile) => ifile.dir_path()
                }
            };
