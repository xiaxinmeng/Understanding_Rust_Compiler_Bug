
$ cargo install cargo-public-api
$ rustup install nightly
$ git clone https://github.com/bevyengine/bevy.git ; cd bevy
$ cargo public-api -p bevy_asset  | grep AssetPathId
[...]
pub struct bevy_asset::AssetPathId(_, _)
[...]
