use science_clients_rs::*;
 
fn main() -> Result<(), Error> {
    let client = OnOffClient::new(subsystem="science", device="brush");
    client.cli_control()
}