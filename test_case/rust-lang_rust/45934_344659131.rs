
        let container_name = |suffix: &str| {
            // InstanceName is an Ident; comes from the caller
            let mut i = Ident::from(format!("{}{}", InstanceName.as_ref(), suffix));
            i.span.0 = Span::call_site();
            return i
        };

        let InstanceNameFfi  = container_name("Ffi");
        let ModuleName       = container_name("Mod");
        let ClassName        = container_name("Class");
        let PrivateClassName = container_name("ClassPrivate");
        let InstanceExt      = container_name("Ext");
