bash
cargo bisect-rustc --access=github --start=1.66.0 --end=2022-12-10 -- build -p=gotham_examples_handlers_simple_async_handlers_await
