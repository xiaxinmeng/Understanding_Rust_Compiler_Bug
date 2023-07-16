
spawn_listener {|port|
    while true {
        alt comm::recv(port) {
               ....
        }
    }
}
