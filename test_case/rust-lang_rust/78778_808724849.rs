rust
            let mut cargo = tool::prepare_tool_cargo(
                builder,
                compiler,
                Mode::ToolRustc,
                host,
                "run",
                "src/tools/miri/cargo-miri",
                SourceType::Submodule,
                &[],
            );
