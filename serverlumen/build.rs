use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  let out_dir = PathBuf::from(env::var("OUT_DIR")?);

  let current_dir = env::current_dir()?;
  let proto_path = current_dir.join("./protolumen/v1/protolumen.proto");
  let parent_path = proto_path
    .parent()
    .expect("proto file should have a parent directory");
  if !proto_path.exists() {
    panic!("[! proto file doesn't exist] {:?}", proto_path);
  }

  tonic_prost_build::configure()
    .file_descriptor_set_path(out_dir.join("protolumen_v1_file_descriptor_set.bin"))
    .compile_protos(&[proto_path.as_path()], &[parent_path])?;

  tonic_prost_build::compile_protos(proto_path)?;

  Ok(())
}
