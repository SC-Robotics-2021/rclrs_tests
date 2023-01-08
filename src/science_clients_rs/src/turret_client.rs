use science_clients_rs::*;
 
fn main() -> Result<(), Error> {
    let client = PositionClient::new(subsystem="science", device="turret");
    client.cli_control()
}