
Rust has three kinds: 'noncopyable', 'copyable', and 'sendable'. By
default, type parameters are considered to be noncopyable. You can
annotate them with the `copy` keyword to declare them copyable, and
with the `send` keyword to make them sendable.
