use science_clients_rs::*;
 
fn main() -> Result<(), Error> {
    let client = OnOffClient::new(subsystem="science", device="uv_light");
    client.cli_control()
}