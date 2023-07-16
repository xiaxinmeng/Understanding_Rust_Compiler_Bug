
        assert_eq!(note.format_for_search(&"ssh".to_string(), 10), "\n\u{1b}[32m@0\u{1b}[0m \u{1b}[36m# SSH\u{1b}[0m \u{1b}[2m(Score: 10)\u{1b}[0m \n\n\u{1b}[2m3. \u{1b}[0m A note about \u{1b}[33mSSH\u{1b}[0m")
