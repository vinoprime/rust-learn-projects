mod basics;
mod mini_projects;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, world!");
//     // basics::main();
//     // mini_projects::main();

//     let addr = "[::1]:50051".parse()?;

//     let btc_service: BitcoinService = BitcoinService::default();

//     Server::builder().add_service(BitcoinServer::new(btc_service)).serve(addr).await?;

//     Ok(())
// }

fn main() {
    println!("Hello, world!");
    basics::main();
    // mini_projects::main();
}
