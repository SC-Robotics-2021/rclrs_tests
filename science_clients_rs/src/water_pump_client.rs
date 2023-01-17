use science_clients_rs::OnOffClient;

fn main() {
    let device = String::from("water_pump");
    let client = OnOffClient::new(device).unwrap();
    let _ = client.cli_control();
}