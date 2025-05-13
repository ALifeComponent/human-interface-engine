use super::proto::generated::greeter_service_server::{GreeterService, GreeterServiceServer};
use super::proto::generated::{SayHelloRequest, SayHelloResponse};

use bevy::prelude::*;

use tonic::{Request, Response, Status, transport::Server};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        let name = request.into_inner().name;
        let reply = SayHelloResponse {
            message: format!("こんにちは、{}さん！", name),
        };
        Ok(Response::new(reply))
    }
}

pub async fn serve_grpc(addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}

pub fn spawn_grpc_request_system(mut commands: Commands) {
    // Bevy の起動直後に gRPC サーバを別スレッドで稼働させる
    let addr = self.addr;
    thread::spawn(move || {
        // 独自 Tokio ランタイムを使って非同期ブロックを実行
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            grpc::serve_grpc(addr)
                .await
                .expect("gRPC サーバの起動に失敗しました");
        });
    });
    info!("gRPC server thread spawned on {}", self.addr);
}
