
    let fancy_ref = &(&mut fancy);
                    ^use '&mut (&mut fancy)' for make mutable
    fancy_ref.num = 6; //~ ERROR E0389
