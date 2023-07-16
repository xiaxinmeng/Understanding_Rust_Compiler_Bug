
309 | static RT: Lazy<Mutex<Runtime>> = Lazy::new(Default::default);
    |                                   --------- ^^^^^^^^^^^^^^^^ the trait `~const Default` is not implemented for `Mutex<Runtime>`
    |                                   required by a bound introduced by this call
