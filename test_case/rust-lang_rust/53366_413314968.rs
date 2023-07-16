rust
            match self.inner.state.compare_exchange(EMPTY, NOTIFIED, SeqCst, SeqCst) {
                Ok(_) => return, // no one was waiting
                Err(NOTIFIED) => return, // already unparked <--
                Err(PARKED) => {} // gotta go wake someone up
                _ => panic!("inconsistent state in unpark"),
            }
