
        if builder.no_std(target) != Some(true) { // Skip only if it's a genuine no_std target
            builder.ensure(compile::Test { compiler, target });
        }
