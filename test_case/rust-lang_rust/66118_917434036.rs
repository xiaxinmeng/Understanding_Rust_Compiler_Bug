rust
use tokio::runtime::Builder;
use tonic::client::Grpc;
use tonic::codec::ProstCodec;
use tonic::codegen::http::uri::PathAndQuery;
use tonic::transport::Channel;
use tonic::IntoRequest;

#[derive(::prost::Message)]
pub struct Proto;

fn main() {
    let f = async {
        let client = unsafe { &mut *(0x1usize as *mut Grpc<Channel>) };
        let _: Result<tonic::Response<Proto>, tonic::Status> = client
            .unary(
                Proto.into_request(),
                PathAndQuery::from_static(""),
                ProstCodec::<Proto, Proto>::default(),
            )
            .await;
    };
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(f);
}
