
class socket_handle {
    let sockfd: int;
    new(x: int) {self.sockfd = x;}
    drop {io::println("hmm");}
}
