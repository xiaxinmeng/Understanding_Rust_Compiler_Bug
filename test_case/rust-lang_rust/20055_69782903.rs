
# For reference, this command produces an invalid binary,
# but minor tweaks (e.g. removing the sroa pass at the end)
# produces a binary that runs without an error.
CMD_1="rustc /tmp/coerce-match-calls2.rs \
      -C no-prepopulate-passes \
      -C passes=datalayout  \
      -C passes=notti  \
      -C passes=targetlibinfo \
      -C passes=no-aa \
      -C passes=tbaa \
      -C passes=scoped-noalias \
      -C passes=assumption-tracker \
      -C passes=basicaa \
      -C passes=ipsccp \
      -C passes=globalopt \
      -C passes=deadargelim \
      -C passes=instcombine \
      -C passes=simplifycfg \
      -C passes=basiccg \
      -C passes=prune-eh \
      -C passes=inline-cost \
      -C passes=inline \
      -C passes=functionattrs \
      -C passes=sroa \
"

# This is a reduced version of the above; it still produces a binary
# that will error in the midst of jemalloc, due pnkfelix thinks to a
# spuriously injected call to `je_sdallocx` (i.e. `free`)
CMD_2="rustc /tmp/coerce-match-calls2.rs \
      -C no-prepopulate-passes \
      -C passes=datalayout  \
      -C passes=instcombine \
      -C passes=inline \
      -C passes=sroa \
"
