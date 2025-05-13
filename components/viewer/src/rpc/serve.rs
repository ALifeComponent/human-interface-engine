use super::proto::generated::SayHelloRequest;
use super::proto::generated::greeter_service_client::GreeterServiceClient;
use bevy::prelude::*;

use std::sync::mpsc::{Receiver, channel};

#[derive(Resource)]
struct HelloReceiver(Receiver<String>);

pub fn spawn_grpc_request_system(mut commands: Commands) {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build Tokio runtime");
        let message = rt.block_on(async {
            let mut client = GreeterServiceClient::connect("http://[::1]:50051")
                .await
                .expect("connect failed");
            let resp = client
                .say_hello(SayHelloRequest {
                    name: "World".into(),
                })
                .await
                .expect("rpc failed")
                .into_inner();
            resp.message
        });
        tx.send(message).expect("send failed");
    });
    commands.insert_resource(HelloReceiver(rx));
}
