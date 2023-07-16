rust

            match &packet {
                Packet::HSBHandshake(ref _handshake) => {
                    // handle here when implemented
                }
                Packet::HSBLegacyStatusPing(ref _ping) => {
// the line under here causes the ICE;
// I don't know why, but that's all I've gotten so far
//
// Leaving it out (commenting it away) gets rid of the issue
                    packet.handle(self).await?;
                }
                _ => unreachable!(
                    "only a handshake or a legacy packet should
                be possible here. any other packet should have returned an
                error of ProtocolError::UnknownPacket."
                ),
            }