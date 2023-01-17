use science_clients_rs::CameraClient;
 
fn main() {
    let subsystem="science";
    let device = "microscope";
    let client = CameraClient::new(subsystem, device).unwrap();
    let _ = client.cli_control();
}