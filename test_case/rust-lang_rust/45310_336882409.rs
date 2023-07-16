
$ llc -march=aarch64 -mattr=help
Available CPUs for this target:

  cortex-a35   - Select the cortex-a35 processor.
  cortex-a53   - Select the cortex-a53 processor.
  cortex-a57   - Select the cortex-a57 processor.
  cortex-a72   - Select the cortex-a72 processor.
  cortex-a73   - Select the cortex-a73 processor.
  cyclone      - Select the cyclone processor.
  exynos-m1    - Select the exynos-m1 processor.
  exynos-m2    - Select the exynos-m2 processor.
  exynos-m3    - Select the exynos-m3 processor.
  falkor       - Select the falkor processor.
  generic      - Select the generic processor.
  kryo         - Select the kryo processor.
  thunderx     - Select the thunderx processor.
  thunderx2t99 - Select the thunderx2t99 processor.
  thunderxt81  - Select the thunderxt81 processor.
  thunderxt83  - Select the thunderxt83 processor.
  thunderxt88  - Select the thunderxt88 processor.

Available features for this target:

  a35                                - Cortex-A35 ARM processors.
  a53                                - Cortex-A53 ARM processors.
  a57                                - Cortex-A57 ARM processors.
  a72                                - Cortex-A72 ARM processors.
  a73                                - Cortex-A73 ARM processors.
  alternate-sextload-cvt-f32-pattern - Use alternative pattern for sextload convert to f32.
  arith-bcc-fusion                   - CPU fuses arithmetic+bcc operations.
  arith-cbz-fusion                   - CPU fuses arithmetic + cbz/cbnz operations.
  balance-fp-ops                     - balance mix of odd and even D-registers for fp multiply(-accumulate) ops.
  crc                                - Enable ARMv8 CRC-32 checksum instructions.
  crypto                             - Enable cryptographic instructions.
  custom-cheap-as-move               - Use custom code for TargetInstrInfo::isAsCheapAsAMove().
  cyclone                            - Cyclone.
  disable-latency-sched-heuristic    - Disable latency scheduling heuristic.
  exynosm1                           - Samsung Exynos-M1 processors.
  exynosm2                           - Samsung Exynos-M2/M3 processors.
  falkor                             - Qualcomm Falkor processors.
  fp-armv8                           - Enable ARMv8 FP.
  fullfp16                           - Full FP16.
  fuse-aes                           - CPU fuses AES crypto operations.
  fuse-literals                      - CPU fuses literal generation operations.
  kryo                               - Qualcomm Kryo processors.
  lse                                - Enable ARMv8.1 Large System Extension (LSE) atomic instructions.
  lsl-fast                           - CPU has a fastpath logical shift of up to 3 places.
  neon                               - Enable Advanced SIMD instructions.
  no-neg-immediates                  - Convert immediates and instructions to their negated or complemented equivalent when the immediate does not fit in the encoding..
  perfmon                            - Enable ARMv8 PMUv3 Performance Monitors extension.
  predictable-select-expensive       - Prefer likely predicted branches over selects.
  ras                                - Enable ARMv8 Reliability, Availability and Serviceability Extensions.
  rdm                                - Enable ARMv8.1 Rounding Double Multiply Add/Subtract instructions.
  reserve-x18                        - Reserve X18, making it unavailable as a GPR.
  slow-misaligned-128store           - Misaligned 128 bit stores are slow.
  slow-paired-128                    - Paired 128 bit loads and stores are slow.
  spe                                - Enable Statistical Profiling extension.
  strict-align                       - Disallow all unaligned memory access.
  sve                                - Enable Scalable Vector Extension (SVE) instructions.
  thunderx                           - Cavium ThunderX processors.
  thunderx2t99                       - Cavium ThunderX2 processors.
  thunderxt81                        - Cavium ThunderX processors.
  thunderxt83                        - Cavium ThunderX processors.
  thunderxt88                        - Cavium ThunderX processors.
  use-aa                             - Use alias analysis during codegen.
  use-postra-scheduler               - Schedule again after register allocation.
  use-reciprocal-square-root         - Use the reciprocal square root approximation.
  v8.1a                              - Support ARM v8.1a instructions.
  v8.2a                              - Support ARM v8.2a instructions.
  zcm                                - Has zero-cycle register moves.
  zcz                                - Has zero-cycle zeroing instructions.

Use +feature to enable a feature, or -feature to disable it.
For example, llc -mcpu=mycpu -mattr=+feature1,-feature2
'+help' is not a recognized feature for this target (ignoring feature)
