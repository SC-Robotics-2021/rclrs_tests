use science_clients_rs::PositionClient;
 
fn main() {
    let client = PositionClient::new(subsystem="science", device="stepper_motor");
    client.cli_control();
}