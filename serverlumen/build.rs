fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tonic
    let current_dir = std::env::current_dir()?;
    let proto_path = current_dir.join("../protolumen/v1/protolumen.proto");
    if !proto_path.exists() {
        panic!("[! proto file doesn't exist] {:?}", proto_path);
    }
    tonic_build::compile_protos(proto_path)?;

    // Clorinde
    let mut client = postgres::Client::connect(&std::env::var("DATABASE_URL")?, postgres::NoTls)?;
    let config = clorinde::config::Config::builder()
        .name("queries")
        .destination("queries/build")
        .build();
    clorinde::gen_live(&mut client, config)?;

    Ok(())
}
