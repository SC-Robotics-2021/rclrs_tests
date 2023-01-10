use science_clients_rs::OnOffClient;

fn main() {
    let client = OnOffClient::new(subsystem="science", device="brush");
    client.cli_control();
}