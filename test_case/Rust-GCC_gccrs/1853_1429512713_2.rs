yaml
      - uses: actions/checkout@v3
        if: ${{ github.base_ref == 'gcc-patch-dev' }} # master commits don't need the gccrs prefix
