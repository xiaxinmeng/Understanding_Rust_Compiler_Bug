rust
bytes.copy_in_place(9, 0, 4);

// Or with the API SimonSapin suggested:
bytes.copy_in_place(9..13, 0);
