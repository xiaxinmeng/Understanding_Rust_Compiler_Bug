
Available CPUs for this target:

  amdfam10      - Select the amdfam10 processor.
  athlon        - Select the athlon processor.
  athlon-4      - Select the athlon-4 processor.
  athlon-fx     - Select the athlon-fx processor.
  athlon-mp     - Select the athlon-mp processor.
  athlon-tbird  - Select the athlon-tbird processor.
  athlon-xp     - Select the athlon-xp processor.
  athlon64      - Select the athlon64 processor.
  athlon64-sse3 - Select the athlon64-sse3 processor.
  atom          - Select the atom processor.
  bdver1        - Select the bdver1 processor.
  bdver2        - Select the bdver2 processor.
  btver1        - Select the btver1 processor.
  btver2        - Select the btver2 processor.
  c3            - Select the c3 processor.
  c3-2          - Select the c3-2 processor.
  core-avx-i    - Select the core-avx-i processor.
  core-avx2     - Select the core-avx2 processor.
  core2         - Select the core2 processor.
  corei7        - Select the corei7 processor.
  corei7-avx    - Select the corei7-avx processor.
  generic       - Select the generic processor.
  geode         - Select the geode processor.
  i386          - Select the i386 processor.
  i486          - Select the i486 processor.
  i586          - Select the i586 processor.
  i686          - Select the i686 processor.
  k6            - Select the k6 processor.
  k6-2          - Select the k6-2 processor.
  k6-3          - Select the k6-3 processor.
  k8            - Select the k8 processor.
  k8-sse3       - Select the k8-sse3 processor.
  knl           - Select the knl processor.
  nehalem       - Select the nehalem processor.
  nocona        - Select the nocona processor.
  opteron       - Select the opteron processor.
  opteron-sse3  - Select the opteron-sse3 processor.
  penryn        - Select the penryn processor.
  pentium       - Select the pentium processor.
  pentium-m     - Select the pentium-m processor.
  pentium-mmx   - Select the pentium-mmx processor.
  pentium2      - Select the pentium2 processor.
  pentium3      - Select the pentium3 processor.
  pentium3m     - Select the pentium3m processor.
  pentium4      - Select the pentium4 processor.
  pentium4m     - Select the pentium4m processor.
  pentiumpro    - Select the pentiumpro processor.
  prescott      - Select the prescott processor.
  westmere      - Select the westmere processor.
  winchip-c6    - Select the winchip-c6 processor.
  winchip2      - Select the winchip2 processor.
  x86-64        - Select the x86-64 processor.
  yonah         - Select the yonah processor.

Available features for this target:

  3dnow                - Enable 3DNow! instructions.
  3dnowa               - Enable 3DNow! Athlon instructions.
  64bit                - Support 64-bit instructions.
  64bit-mode           - 64-bit mode (x86_64).
  adx                  - Support ADX instructions.
  aes                  - Enable AES instructions.
  atom                 - Intel Atom processors.
  avx                  - Enable AVX instructions.
  avx-512              - Enable AVX-512 instructions.
  avx-512-cdi          - Enable AVX-512 Conflict Detection Instructions.
  avx-512-eri          - Enable AVX-512 Exponential and Reciprocal Instructions.
  avx-512-pfi          - Enable AVX-512 PreFetch Instructions.
  avx2                 - Enable AVX2 instructions.
  bmi                  - Support BMI instructions.
  bmi2                 - Support BMI2 instructions.
  call-reg-indirect    - Call register indirect.
  cmov                 - Enable conditional move instructions.
  cmpxchg16b           - 64-bit with cmpxchg16b.
  f16c                 - Support 16-bit floating point conversion instructions.
  fast-unaligned-mem   - Fast unaligned memory access.
  fma                  - Enable three-operand fused multiple-add.
  fma4                 - Enable four-operand fused multiple-add.
  fsgsbase             - Support FS/GS Base instructions.
  hle                  - Support HLE.
  idiv-to-divb         - Use small divide for positive values less than 256.
  lea-sp               - Use LEA for adjusting the stack pointer.
  lea-uses-ag          - LEA instruction needs inputs at AG stage.
  lzcnt                - Support LZCNT instruction.
  mmx                  - Enable MMX instructions.
  movbe                - Support MOVBE instruction.
  pad-short-functions  - Pad short functions.
  pclmul               - Enable packed carry-less multiplication instructions.
  popcnt               - Support POPCNT instruction.
  prfchw               - Support PRFCHW instructions.
  rdrand               - Support RDRAND instruction.
  rdseed               - Support RDSEED instruction.
  rtm                  - Support RTM instructions.
  slow-bt-mem          - Bit testing of memory is slow.
  sse                  - Enable SSE instructions.
  sse2                 - Enable SSE2 instructions.
  sse3                 - Enable SSE3 instructions.
  sse41                - Enable SSE 4.1 instructions.
  sse42                - Enable SSE 4.2 instructions.
  sse4a                - Support SSE 4a instructions.
  ssse3                - Enable SSSE3 instructions.
  vector-unaligned-mem - Allow unaligned memory operands on vector/SIMD instructions.
  xop                  - Enable XOP instructions.

Use +feature to enable a feature, or -feature to disable it.
For example, llc -mcpu=mycpu -mattr=+feature1,-feature2
