use science_clients_rs::OnOffClient;

fn main() {
    let client = OnOffClient::new(subsystem="science", device="brush").unwrap();
    let _ = client.cli_control();
}