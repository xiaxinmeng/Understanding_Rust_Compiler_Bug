
trait Hand {
    type SendMessageType;
    type ReceiveMessageType;

    fn send_message(&mut self, message: Self::SendMessageType) -> Self::ReceiveMessageType;
}

trait MeldHand: Hand {}

impl<Meld: MeldHand> Hand for Meld {
    type SendMessageType = ();
    type ReceiveMessageType = ();

    fn send_message(&mut self, _message: Self::SendMessageType) -> Self::ReceiveMessageType {}
}
