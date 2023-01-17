use science_clients_rs::OnOffClient;
 
fn main() {
    let subsystem="science";
    let device = "vacuum";
    let client = OnOffClient::new(subsystem, device).unwrap();
    let _ = client.cli_control();
}