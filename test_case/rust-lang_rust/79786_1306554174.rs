shell
# clone the repo
git clone https://github.com/datafuselabs/databend.git
# cd into this crate
cd src/query/functions-v2
# The first check will work
cargo check
# Change any bit in `functions-v2`
# For example, replace `"tuple".to_string()` to `"tuple ".to_string()`
# The second one will always fail in ICE.
cargo check
