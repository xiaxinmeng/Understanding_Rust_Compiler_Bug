 rust
fn eat(&self, table: &Table){
    println!("{} is reaching for the {} fork",self.name,self.left);
    let _left = table.forks[self.left].lock().unwrap();
    println!("{} got the {} fork!",self.name, self.left);

    println!("{} pauses to compose themselves.", self.name);
    thread::sleep_ms(100);

    println!("{} is reaching for the {} fork",self.name,self.right);
    let _right = table.forks[self.right].lock().unwrap();
    println!("{} got the {} fork!", self.name,self.right);

    println!("{} got the forks! Time to eat.", self.name);
    thread::sleep_ms(1000);
    println!("{} is done eating.", self.name);
}
