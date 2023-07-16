
    |
800 |                     errs.iter().map(|&(sp, _)| sp).collect::<Vec<Span>>(),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax_pos::Span`, found struct `std::vec::Vec`
    |
    = note: expected type `syntax_pos::Span`
               found type `std::vec::Vec<syntax_pos::Span>`
