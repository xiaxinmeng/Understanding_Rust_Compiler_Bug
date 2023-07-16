rust

fn foo(){
    #[cfg(target_pointer_width = "32")]
    {
        // 32 bit impl
    }

    #[cfg(target_pointer_width = "64")]
    {
        // 64 bit impl
    }
}

