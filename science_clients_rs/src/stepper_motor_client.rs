use science_clients_rs::PositionClient;
 
fn main() {
    let subsystem="science";
    let device = "stepper_motor";
    let client = PositionClient::new(subsystem, device).unwrap();
    let _ = client.cli_control();
}