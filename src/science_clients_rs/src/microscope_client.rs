use science_clients_rs::*;
 
fn main() -> Result<(), Error> {
    let client = CameraClient::new(subsystem="science", device="microscope");
    client.cli_control()
}