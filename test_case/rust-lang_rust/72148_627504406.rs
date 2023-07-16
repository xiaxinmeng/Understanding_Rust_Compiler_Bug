
    {
        let r: Box<dyn Error> = "oh no".into();
        std::mem::drop(r);
    }
