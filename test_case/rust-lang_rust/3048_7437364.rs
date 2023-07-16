
trait option_extensions for option<~str> {

    /// if the specified option is some(msg), then fail with msg
    fn fail_if_some () {
        for self.each |msg| { fail msg }
    }

}
