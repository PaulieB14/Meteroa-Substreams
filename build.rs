fn main() {
    prost_build::compile_protos(&["proto/meteora.proto"], &["proto"]).unwrap();
}
