
        if !target.contains("-none-") { // Skip only if it's a "-none-" target
            builder.ensure(compile::Test { compiler, target });
        }
