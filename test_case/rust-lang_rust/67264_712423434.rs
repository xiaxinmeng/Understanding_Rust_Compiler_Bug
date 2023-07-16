rust
    match delta {
        (0, 0, 0, 0) => "just now".to_string(),
        (0, 0, 0, seconds) => format!("{} seconds ago", seconds),
        (0, 0, 1, ..) => "one minute ago".to_string(),
        (0, 0, minutes, ..) => format!("{} minutes ago", minutes),
        (0, 1, ..) => "an hour ago".to_string(),
        (0, hours, ..) => format!("{} hours ago", hours),
        (1, ..) => "one day ago".to_string(),
        (days @ 2..=5, ..) => format!("{} days ago", days),
        _ => format!("{}", tm.strftime("%b %d, %Y").unwrap()),
    }
