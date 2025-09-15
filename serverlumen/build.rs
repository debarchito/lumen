fn main() -> Result<(), Box<dyn std::error::Error>> {
  let current_dir = std::env::current_dir()?;
  let proto_path = current_dir.join("./protolumen/v1/protolumen.proto");
  if !proto_path.exists() {
    panic!("[! proto file doesn't exist] {:?}", proto_path);
  }

  tonic_prost_build::compile_protos(proto_path)?;

  Ok(())
}
