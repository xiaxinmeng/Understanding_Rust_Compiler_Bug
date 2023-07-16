
        if !target.contains("-none-") {
            builder.ensure(compile::Test { compiler, target });
        }
