rust
// Every endpoint in the Matrix REST API has two request and response types in Ruma, one Incoming
// (used for deserialization) and out Outgoing (used for serialization). To avoid annoying clones when
// sending a request, most non-copy fields in the outgoing structs are references.
//
// The request traits have an associated type for the corresponding response type so things can be
// matched up properly.
pub trait IncomingRequest: Sized {
    // This is the current definition of the associated type I'd like to make generic.
    type OutgoingResponse: OutgoingResponse;
    // AFAICT adding a lifetime parameter is all I need.
    type OutgoingResponse<'a>: OutgoingResponse;

    // Other trait members... (not using Self::OutgoingResponse)
}
