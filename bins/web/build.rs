fn main() {
    tonic_build::configure()
        .extern_path(
            ".monitor_core_types.Metric",
            "::monitor::monitor_core_types::Metric",
        )
        .extern_path(
            ".monitor_core_types.Log",
            "::monitor::monitor_core_types::Log",
        )
        .compile(
            &["proto/web_socket.proto"], // Protos
            &["../../proto", "."] // Include paths
        ).unwrap();
}
