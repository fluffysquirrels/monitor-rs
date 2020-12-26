fn main() {
    tonic_build::compile_protos("proto/web_socket.proto").unwrap();
}
