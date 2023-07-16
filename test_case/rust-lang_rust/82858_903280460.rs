rust
trait AncillaryMessage {
   fn to_raw(&self) -> (c_int, c_int, &[u8]);

   // and some unsafe helper methods do the memcpy to custom types properly
}

struct RawAncillaryMessage {
   // I'm not 100% sure, but I think for raw accessors to platform-specific values using `c_int` should be fine.
   fn from_raw(level: c_int, type: c_int, body: &[u8]) -> Self;
}

impl AncillaryMessage for RawAncillaryMessage {
  ...
}

impl AncillaryMessage for IpAncillaryData {
  ...
}

impl TryFrom<RawAncillaryMessage> for IpAncillaryData {
  ...
}

impl SocketAncillary {
  pub fn addMessage(&mut self, impl AncillaryMessage) {
     ...
  }

  pub fn getMessages(&self) -> impl Iterator<Item = RawAncillaryMessage> {
     ...
  }
}
