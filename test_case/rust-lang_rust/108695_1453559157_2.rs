rust
// ./src/utils/connection.rs

pub fn connection() {
    #[path = './messagebox.rs']
    mod messagebox;

    messagebox::print_message(&something);
}
