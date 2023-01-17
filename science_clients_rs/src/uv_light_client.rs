use science_clients_rs::OnOffClient;
 
fn main() {
    let client = OnOffClient::new(subsystem="science", device="uv_light").unwrap();
    let _ = client.cli_control();
}