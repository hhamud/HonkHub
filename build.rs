use eyre::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=./protobufs");

    let src = PathBuf::from("./protobufs");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .protoc_arg("--experimental_allow_proto3_optional")
        .protoc_arg("--proto_path=./protobufs")
        .compile(
            &[
                src.join("gossip.proto"),
                src.join("hub_event.proto"),
                src.join("hub_state.proto"),
                src.join("job.proto"),
                src.join("message.proto"),
                src.join("onchain_event.proto"),
                src.join("request_response.proto"),
                src.join("rpc.proto"),
                src.join("sync_trie.proto"),
                src.join("username_proof.proto"),
            ],
            &["protobufs"],
        )?;

    Ok(())
}
