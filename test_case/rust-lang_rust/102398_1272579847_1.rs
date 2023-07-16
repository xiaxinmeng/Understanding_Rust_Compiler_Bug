
                    if !std::thread::panicking() {
                        std::thread::park_timeout(SLEEP_DURATION);
                    }
