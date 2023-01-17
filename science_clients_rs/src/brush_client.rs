use science_clients_rs::OnOffClient;

fn main() {
    let subsystem="science";
    let device = "brush";
    let client = OnOffClient::new(subsystem, device).unwrap();
    let _ = client.cli_control();
}