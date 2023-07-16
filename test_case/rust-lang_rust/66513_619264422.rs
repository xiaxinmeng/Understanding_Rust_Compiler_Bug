rust
                if ret == 0 {
                    let err = io::Error::last_os_error();
                    debug!("failed acquiring file lock: {}", err);
                    Err(err)
                } else if ret == ERROR_INVALID_FUNCTION {
                    debug!("locking not supported");
                    Ok(Lock { _file: file })
                } else {
                    debug!("successfully acquired lock");
                    Ok(Lock { _file: file })
                }
