use reqwest::Client;

fn main() {
    let client = Client::new();
    let res = client.get("http://localhost:8888/health").send().unwrap();
    if res.status() == 200 {
        println!("Server is healthy");
    } else {
        println!("Server is not healthy");
    }
}