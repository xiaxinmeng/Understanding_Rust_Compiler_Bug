rust
    extern crate prost_build;

    struct SG;
    impl prost_build::ServiceGenerator for SG {
        fn generate(&self, _: prost_build::Service, _: &mut String) {
        }
    }

    fn main() {
        let mut cfg = prost_build::Config::new();
        cfg.service_generator(Box::new(SG));
        cfg.compile_protos(&["p.proto"], &["."]).unwrap();
    }
    