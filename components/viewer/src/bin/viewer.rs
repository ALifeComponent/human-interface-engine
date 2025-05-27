#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// This is the address where the viewer will connect to the gRPC server.
    /// It should be in the format of "IP:Port".
    #[arg(long, default_value = "127.0.0.1:50051", name = "grpc-addr")]
    grpc_addr: SocketAddr,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    viewer::run_app(cli.grpc_addr)?;

    Ok(())
}
