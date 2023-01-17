use science_clients_rs::PositionClient;
 
fn main() {
    let client = PositionClient::new(subsystem="science", device="stepper_motor");
    let _ = client.cli_control();
}