
run.path("src/tools/cargo").default_condition(
    builder.config.extended
    && builder.config.tools.as_ref().map_or(true, |t| t.contains("cargo")))
