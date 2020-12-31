fn main() {
    tonic_build::configure()
        .extern_path(
            ".monitor_core_types.Metric",
            "::monitor::monitor_core_types::Metric",
        )
        .compile(
            &["proto/web_socket.proto"], // Protos
            &["../../proto", "."] // Include paths
        ).unwrap();
}
