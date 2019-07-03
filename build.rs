use protoc_rust::{self, Customize};

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["protos/encryption_result.proto"],
        includes: &["protos"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc")
}
