
        if builder.no_std(target) == Some(false) {
            builder.ensure(compile::Test { compiler, target });
        }
