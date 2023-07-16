rust
#[link(name = "libtest_time")]
#[link(name = "boost_system")]
extern "C" {
    fn print_time();
}
