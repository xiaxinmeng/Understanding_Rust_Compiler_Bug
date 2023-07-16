
    bb9: {
        _5 = &'_#4r mut (((*_1) as Ok).0: u32); // bb9[2]: scope 0 at /home/nmatsakis/tmp/issue-51348.rs:5:12: 5:13
        _6 = &'_#5r _5;                  // bb9[3]: scope 0 at /home/nmatsakis/tmp/issue-51348.rs:5:12: 5:13
        ...
    }

    ...

    bb12: {
        _5 = &'_#7r mut (((*_1) as Err).0: u32); // bb12[2]: scope 0 at /home/nmatsakis/tmp/issue-51348.rs:5:21: 5:22
        _6 = &'_#8r _5;                  // bb12[3]: scope 0 at /home/nmatsakis/tmp/issue-51348.rs:5:21: 5:22
        ...
    }
