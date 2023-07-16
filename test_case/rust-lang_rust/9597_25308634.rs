 rust
trait Walker {  // Some business-logic trait
    fn walk(&mut self);
}

// A struct which is intended to be an implementor of Walker trait
// Note that it has lifetime parameter in order to work for any kind
// of pointer to a Reader
struct ReaderContainer {
    priv reader: @Reader,
    priv counter: int
}

// Some auxiliary structure for ReaderContainer
// It may be anything but it should have a reference to ReaderContainer
// We have to use lifetime parameter because this structure is 'attached'
// to ReaderContainer, hence it must be of the same lifetime
struct ReaderContainerIterator<'self> {
    priv container: &'self mut ReaderContainer
}

// Some made-up implementation of iterator protocol for our
// auxiliary structure, it does not really matter
impl<'self> Iterator<u8> for ReaderContainerIterator<'self> {
    fn next(&mut self) -> Option<u8> {
        if self.container.counter < 10 {
            self.container.counter += 1;
            Some(self.container.reader.read_byte() as u8)
        } else {
            None
        }
    }
}

impl ReaderContainer {
    // A constructor for ReaderContainer, nothing special
    fn new(reader: @Reader) -> ReaderContainer {
        ReaderContainer { reader: reader, counter: 0 }
    }

    // A method which returns our auxiliary structure, i.e. iterator
    // Note that self parameter has lifetime 'self, otherwise this naturally
    // does not compile
    fn iter<'r>(&'r mut self) -> ReaderContainerIterator<'r> {
        ReaderContainerIterator { container: self }
    }
}

// Here is the problem: we cannot implement Walker trait!
impl Walker for ReaderContainer {
    // See below for concrete errors description
    fn walk(&mut self) {  // <<<
        for b in self.iter() {
            println(fmt!("byte %?", b));
        }
    }
}

fn main() {
    use std::io;

    let r = io::stdin();
    let mut c = ReaderContainer::new(r);

    c.walk();
}
