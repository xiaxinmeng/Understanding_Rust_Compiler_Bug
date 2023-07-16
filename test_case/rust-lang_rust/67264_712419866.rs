rust
    // on stable
    match delta {
        (days, ..) if days > 5 => format!("{}", tm.strftime("%b %d, %Y").unwrap()),
        (days @ 2..=5, ..) => format!("{} days ago", days),
        (1, ..) => "one day ago".to_string(),

        (0, hours, ..) if hours > 1 => format!("{} hours ago", hours),
        (0, 1, ..) => "an hour ago".to_string(),

        (0, 0, minutes, _) if minutes > 1 => format!("{} minutes ago", minutes),
        (0, 0, 1, _) => "one minute ago".to_string(),

        (0, 0, 0, seconds) if seconds > 0 => format!("{} seconds ago", seconds),
        _ => "just now".to_string(),
    };
    // on nightly
    match delta {
        (6.., ..) => format!("{}", tm.strftime("%b %d, %Y").unwrap()),
        (days @ 2..=5, ..) => format!("{} days ago", days),
        (1, ..) => "one day ago".to_string(),

        (0, hours @ 2.., ..) => format!("{} hours ago", hours),
        (0, 1, ..) => "an hour ago".to_string(),

        (0, 0, minutes @ 2.., _) => format!("{} minutes ago", minutes),
        (0, 0, 1, _) => "one minute ago".to_string(),

        (0, 0, 0, seconds @ 1..) => format!("{} seconds ago", seconds),
        _ => "just now".to_string(),
    }
