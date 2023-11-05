fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("I am from build.rs");
    tonic_build::compile_protos("./src/mini_projects/protos/payments.proto")?;
    Ok(())
}
