fn main() {
    tonic_build::compile_protos("proto/collector.proto").unwrap();
}
