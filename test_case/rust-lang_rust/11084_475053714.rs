
% for VERSION in 1.{13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33}.0 ; do (\
  echo $VERSION && rustup update $VERSION && rustc +$VERSION --version && \
  RUSTC_BOOTSTRAP=1 rustc +$VERSION -C opt-level=3  --test main-bench.rs && \
   ./main-bench --bench && ./main-bench --bench && ./main-bench --bench && ./main-bench --bench && ./main-bench --bench \
) ; done | tee version-bench.txt
