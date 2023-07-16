cmake
# Assume we are using `cmake`
add_custom_target(
    ...
    COMMAND SNEAK_DOLLAR_MAKE_INTO_THE_COMMAND=$(MAKE) cargo build
    ...
)
