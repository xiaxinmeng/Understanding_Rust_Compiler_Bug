sh
nm -S toolchain/lib/librustc_driver-*.so \
  | sed -E 's/^[0-9a-f]* //;s/^ *([a-zA-Z] )/---------------- \1/' \
  | sort -k 3 \
  > rustc_driver.syms
