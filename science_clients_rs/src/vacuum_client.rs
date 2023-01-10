use science_clients_rs::*;
 
fn main() -> Result<(), Error> {
    let client = OnOffClient::new(subsystem="science", device="vacuum");
    client.cli_control()
}