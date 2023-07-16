
>  #[inline]
>  #[stable(feature = "needs_drop", since = "1.21.0")]
> +#[rustc_const_unstable(feature = "const_needs_drop")]
> +#[cfg(not(stage0))]
> +pub const fn needs_drop<T>() -> bool {
> +    intrinsics::needs_drop::<T>()
> +}
> +
> +#[inline]
> +#[stable(feature = "needs_drop", since = "1.21.0")]
> +#[cfg(stage0)]
> +/// Ceci n'est pas la documentation
>
> Thanks for the heads up. If #54601
> <https://github.com/rust-lang/rust/pull/54601> merges first I'll be sure
> to sync and test with and without the dummy non-const version.
>
> —
> You are receiving this because you commented.
> Reply to this email directly, view it on GitHub
> <https://github.com/rust-lang/rust/pull/54596#discussion_r221133842>, or mute
> the thread
> <https://github.com/notifications/unsubscribe-auth/AApc0gjk-UOOYBNpNtTWAXKJJTB7bwbXks5ufZh7gaJpZM4W7Y3o>
> .
>
