
// Use one template module to specify in a single file the implementation
// of functions for multiple types

mod mystd {

    // The template on type T is specified in float-template.rs
    #[path = "float-template"]
    mod float {
        // The type of the float
        import inst::T;

        // Define T as float
        #[path = "inst_float.rs"]
        mod inst;
    }

    // Use the same template
    #[path = "float-template"]
    mod f64 {

        import inst::T;

        // Define T as f64
        #[path = "inst_f64.rs"]
        mod inst;
    }

    #[path = "float-template"]
    mod f32 {
        import inst::T;

        #[path = "inst_f32.rs"]
        mod inst;
    }

}
