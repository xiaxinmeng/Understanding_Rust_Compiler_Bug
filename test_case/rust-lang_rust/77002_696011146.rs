
~> rustc +nightly -Zmir-opt-level=2 mis.rs -Zfuel=mis=27
warning: optimization-fuel-exhausted: DestinationPropagation DefId(0:11 ~ mis[317d]::main[0]::matrix_prod[0]) CandidateAssignment { dest: _20, src: _19, loc: bb6[3] }

warning: 1 warning emitted

~> ./mis
[[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
~> rustc +nightly -Zmir-opt-level=2 mis.rs -Zfuel=mis=26
warning: optimization-fuel-exhausted: DestinationPropagation DefId(0:11 ~ mis[317d]::main[0]::matrix_prod[0]) CandidateAssignment { dest: _0, src: _6, loc: bb4[8] }

warning: 1 warning emitted

~> ./mis
[[-66035979, 52978690, -50400250, 65261310], [-34738690, 3441401, -16498690, 80122370], [80122370, -94983430, -36313859, -56253950], [43746050, -7369730, -7491330, -48821759]]
