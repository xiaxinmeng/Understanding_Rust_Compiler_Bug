
 env DESTDIR="/some/staging/path" python ./x.py install -vv --config=config.toml \
      --exclude src/tools/miri || die
