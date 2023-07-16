rust
> use std::time::Duration;
>
> let duration = 2569 * Duration::MILLISECOND;
>
> assert_eq!(2, duration.as_secs());
> assert_eq!(569_000_000, duration.subsec_nanos());
> 