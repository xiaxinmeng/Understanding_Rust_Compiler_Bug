
> +/// use std::path::PathBuf;
> +///
> +/// let path = PathBuf::from("c:\\windows\system32.dll");
>
> Here the \\ is a escape sequence in the path string, so the real path *is*
> C:\Windows\ (note the path fragment Windows is capitalized since on
> Windows the name of this directory is capitailized by default).
>
> —
> You are receiving this because you were mentioned.
> Reply to this email directly, view it on GitHub
> <https://github.com/rust-lang/rust/pull/41531#discussion_r113409896>, or mute
> the thread
> <https://github.com/notifications/unsubscribe-auth/AApc0rOkpDhRbXAT7voA5o4N1LyTJDM4ks5rzxNEgaJpZM4NHTJ_>
> .
>
