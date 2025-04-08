fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/protolumen.capnp")
        .output_path("src/schema")
        .run()
        .expect("failed to compile cap'n proto schema");
}
