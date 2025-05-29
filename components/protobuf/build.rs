fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(
            &["../../proto/viewer/v1/viewer.proto"],
            // The path to search for includes
            &["../../proto/"],
        )
        .unwrap();
}
