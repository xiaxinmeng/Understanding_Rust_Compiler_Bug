
pub fn sum(input: &[u32]) -> u32 {
    pub extern crate runtime_target_feature_rt as rt;

    static PTR: rt::atomic::Atomic<fn (&[u32]) -> u32> = rt::atomic::Atomic::new(setup);

    fn setup(input: &[u32]) -> u32 {
        let chosen_function = if rt::have_avx( ) { // have_avx reads cpuid
            enable_avx
        } else {
            default
        };
        PTR.store(chosen_function, rt::atomic::Ordering::Relaxed);
        chosen_function(input)
    }

    fn default(input: &[u32]) -> u32 {
        #[cfg(target_feature = "avx")]
        { /* write some assembly using avx */ }

        #[cfg(not(target_feature = "avx"))]
        { /* fallback code */ input.iter().sum()} // I would like this compiled here
    }

    #[target_feature = "+avx"]
    fn enable_avx(input: &[u32] ) -> u32 {
        #[cfg(target_feature = "avx")]
        { /* write some assembly using avx */ } // but this compiled here

        #[cfg(not(target_feature = "avx"))]
        { /* fallback code */ input.iter().sum()}
    }

    PTR.load(rt::atomic::Ordering::Relaxed)(input)
}
