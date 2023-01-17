use science_clients_rs::OnOffClient;
 
fn main() {
    let subsystem="science";
    let device = "uv_light";
    let client = OnOffClient::new(subsystem, device).unwrap();
    let _ = client.cli_control();
}