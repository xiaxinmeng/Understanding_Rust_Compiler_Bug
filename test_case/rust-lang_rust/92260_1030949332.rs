python
    # Read from `RUST_BOOTSTRAP_CONFIG`, then `--config`, then fallback to `config.toml` (if it
    # exists).
    toml_path = os.getenv('RUST_BOOTSTRAP_CONFIG') or args.config
