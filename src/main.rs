use std::env;
use std::process;

#[tokio::main]
async fn send_line(access_token: String, msg: &str) -> String{

    let mut strpass = String::new();
    strpass.push_str(&"Bearer ");
    strpass.push_str(&access_token);

    let mut body_json = String::new();
    body_json.push_str(&"{\"messages\":[{\"type\":\"text\",\"text\":\"");
    body_json.push_str(&msg);
    body_json.push_str("\"}]}");

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.line.me/v2/bot/message/broadcast")
        .header("Authorization", &strpass)
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .await.unwrap();

    return res.text().await.unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let access_token: String = match env::var("LINE_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Error with LINE_ACCESS_TOKEN");
            process::exit(1);
        },
    };
    
    let args: Vec<String> = std::env::args().collect();
    let messages = &args[1..];
    let mut mes: String = String::new();
    if messages.len() >= 1 {
        mes = messages.join(" ");
    } else {
        mes = "From Rust".to_string();
    }

    let _res = send_line(access_token, &mes);
    return Ok(());
}
