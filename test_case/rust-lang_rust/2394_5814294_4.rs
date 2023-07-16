
    iter::repeat(times) -> {
        task::spawn -> {
            iter::repeat(msgs) -> {
                send(ch_a, "a")
            }
        };
        task::spawn -> {
            iter::repeat(msgs) -> {
                send(ch_b, "b")
            }
        };
    }

vs.

    iter::repeat(times) {||
        task::spawn {||
            iter::repeat(msgs) {||
                send(ch_a, "a")
            }
        };
        task::spawn {||
            iter::repeat(msgs) {||
                send(ch_b, "b")
            }
        };
    }
