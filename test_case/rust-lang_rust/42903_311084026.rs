
DEBUG:rustc_borrowck::borrowck::mir::gather_moves: move paths for nodep.rs:13:1: 19:2:
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp0 = MovePath { lvalue: _0 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp1 = MovePath { first_child: mp9, lvalue: _1 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp2 = MovePath { lvalue: _2 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp3 = MovePath { lvalue: _3 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp4 = MovePath { lvalue: _4 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp5 = MovePath { lvalue: _5 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp6 = MovePath { lvalue: _6 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp7 = MovePath { lvalue: _7 }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp8 = MovePath { parent: mp1, lvalue: (_1.0: DropHasLifetime<'_>) }
DEBUG:rustc_borrowck::borrowck::mir::gather_moves:     mp9 = MovePath { parent: mp1, next_sibling: mp8 lvalue: (_1.0: DropHasLifetime) }
