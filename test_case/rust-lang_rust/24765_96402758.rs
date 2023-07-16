 text
% rustc --version
rustc 1.1.0-nightly (da623844a 2015-04-25) (built 2015-04-25)

% rustc -C debug-assertions       small.rs  && ./small
enter main
calling foo return-new
E::drop hi1, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo return-new
calling foo pass-thru
E::drop hi foo, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo pass-thru
back in main
E::drop hi2, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
E::drop hi foo, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done

% rustc -C debug-assertions -O       small.rs  && ./small
enter main
calling foo return-new
E::drop hi1, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo return-new
calling foo pass-thru
E::drop hi foo, do_swap: true
Trace/BPT trap: 5

% ./x86_64-apple-darwin/stage2/bin/rustc -C debug-assertions       small.rs  && ./small
enter main
calling foo return-new
E::drop hi1, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo return-new
calling foo pass-thru
E::drop hi foo, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo pass-thru
back in main
E::drop hi2, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
E::drop hi foo, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done

% ./x86_64-apple-darwin/stage2/bin/rustc -C debug-assertions -O       small.rs  && ./small
enter main
calling foo return-new
E::drop hi1, do_swap: true
E::drop replaced, do_swap: false
E::drop done
E::drop done
back from foo return-new
calling foo pass-thru
E::drop hi foo, do_swap: true
Trace/BPT trap: 5

% ./x86_64-apple-darwin/stage2/bin/rustc --version
rustc 1.0.0-dev (b0a480875 2015-04-24) (built 2015-04-24)

% 
