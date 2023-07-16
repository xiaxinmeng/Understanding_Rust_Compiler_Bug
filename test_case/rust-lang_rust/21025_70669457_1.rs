
foo.rs:172:19: 172:24 error: mismatched types:
 expected `Session<Cap<(), Rec<Offer<_, Rcv<String, Choose<Snd<_, _>, Snd<_, Offer<_, Rcv<String, Choose<Snd<(), _>, _>>>>>>>>>, (), _>`,
    found `Session<Cap<(), Rec<Offer<_, Rcv<String, Choose<Snd<_, _>, Snd<_, Offer<_, Rcv<String, Choose<Snd<String, _>, _>>>>>>>>>, (), ()>`
(expected (),
    found struct `String`)
foo.rs:172     connect(cli2, srv());
                              ^~~~~
