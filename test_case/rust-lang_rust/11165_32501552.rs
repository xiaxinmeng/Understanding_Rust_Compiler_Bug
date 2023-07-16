 rust
// Create a selector which is a new object
let selector = Selector::new().unwrap();

// First create a TcpStream object. 
// This is in my current implementation quite the same as the native I/O ip stream
// - a very thin wrapper on top of the FD. It provides the same read and write methods.
let mut nativeStream = TcpStream::connect(socketaddr).unwrap();

nativeStream.set_blocking(false); // Switch to non-blocking I/O if desired

// This is new. It "upgrades" the TcpStream into a SelectableTcpStream object by consuming it. 
// This class features additonal async read/write methods that will be performed by using the 
// associated selector.
// I like that approach quite much because if you don't want async I/O you don't need to use it.
let mut selectableStream = stream.associate_selector(&selector);

// By performing selectableStream.disassociate_selector() the original TcpStream can be restored.
// This will only work when no async I/O operation is pending

// Start an async I/O operations. 
// In contrast to the existing APIs the operation needs an owned buffer 
// in order to manipulate it in the background.
let handle = selectableStream.read_some_async(
    ByteBuffer { buffer: ~[0, ..100], offset: 0, length: 20}
);
// handle is either an IoResult<uint>, so either an IoError or a uint as a handle.

// Query the selector for finished I/O when required
let result = selector.wait();
// This blocks until one of the started async operations finishes. It also returns an IoResult<uint>
// which contains in case of success the handle of the operation that finished.

// In case the handles match the result of the async operation can be retrieved
match (handle, result) { 
    (Ok(h1),Ok(h2)) if h1 == h2 => {
        let result = stream.end_read_async();
        // This will return either an IoError or the result of the operation which consists of the ByteBuffer
        // that was passed at the start of the operation and the number of bytes that were read.
    },...
}

