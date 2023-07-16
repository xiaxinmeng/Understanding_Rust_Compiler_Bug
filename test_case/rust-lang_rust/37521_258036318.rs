
# Build every libstd possible. That is:
#   * arm/lib/x86_64/libstd
#   * arm/lib/arm/libstd
#   * arm/lib/powerpc/libstd
#   * x86_64/lib/x86_64/libstd
#   * x86_64/lib/arm/libstd
#   * x86_64/lib/powerpc/libstd
./x.py build src/libstd

# Build libstd for all hosts for one target:
#   * arm/lib/arm/libstd
#   * x86_64/lib/arm/libstd
./x.py build src/libstd --target arm

# Build libstd for one host and one target:
#   * arm/lib/arm/libstd
./x.py build src/libstd --target arm --host arm
