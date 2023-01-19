use science_clients_rs::PositionClient;
 
fn main() {
    let device = String::from("stepper_motor");
    let client = PositionClient::new(device).unwrap();
    let _ = client.cli_control();
}