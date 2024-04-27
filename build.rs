fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
       .build_server(true)
       .out_dir("src/generated/protobuf")
       .compile(&["proto/authentication.proto", "proto/role.proto", "proto/user.proto"], &["proto"])
       .unwrap();
    //tonic_build::compile_protos("./proto/authentication.proto")?;
    Ok(())
}