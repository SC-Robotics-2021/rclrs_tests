use science_clients_rs::OnOffClient;

fn main() {
    let device = String::from("uv_light");
    let client = OnOffClient::new(device).unwrap();
    let _ = client.cli_control();
}