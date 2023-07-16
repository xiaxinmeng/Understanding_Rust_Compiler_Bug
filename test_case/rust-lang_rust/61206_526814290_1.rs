
      env $(cat "config.env)\
          python ./x.py build -vv --config=config.toml -j<somenumber> \
          --exclude src/tools/miri
