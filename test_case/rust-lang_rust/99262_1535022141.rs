
   #[repr(align(4))]
   #[derive(Debug)]
   pub(crate) struct SimpleMessage {
       kind: ErrorKind,
       message: &'static str,
   }
   