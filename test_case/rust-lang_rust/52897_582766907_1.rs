
$ uname -a
Linux debian 5.4.0-3-4kc-malta #1 SMP Debian 5.4.13-1 (2020-01-19) mips GNU/Linux
$ cat src/lib.rs 
#[cfg(test)]
mod tests {
    #[test]
    fn min() {
        let nan = 0f64 / 0f64;
        assert_eq!(1f64.min(nan), 1f64);
    }

    #[test]
    fn max() {
        let nan = 0f64 / 0f64;
        assert_eq!(1f64.max(nan), 1f64);
    }
}
