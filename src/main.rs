mod config;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use config::{Config, DnsType};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    r#type: String,
    name: String,
    content: String,
    ttl: i32,
    proxied: bool,
}

async fn fetch_ip(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    match client.get(url).send().await {
        Ok(result) => {
            let ip = result.text().await?;
            if ip.is_empty() {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Empty IP address response",
                )))
            } else {
                Ok(ip)
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let mut config: Config = Config::new();

    println!("Loaded Configuration: {:#?}", config);

    let mut ip = String::new();

    if config.dns.r#type == DnsType::AAAA {
        match fetch_ip(&client, "https://api6.ipify.org").await {
            Ok(ipv6) => ip = ipv6,
            Err(_) => {
                eprintln!("Failed to retrieve IPv6 address. Falling back to IPv4.");
                if let Ok(ipv4) = fetch_ip(&client, "https://api.ipify.org").await {
                    ip = ipv4;
                    config.dns.r#type = DnsType::A;
                } else {
                    eprintln!("Failed to retrieve both IPv6 and IPv4 addresses.");
                    return;
                }
            }
        }
    } else if config.dns.r#type == DnsType::A {
        match fetch_ip(&client, "https://api.ipify.org").await {
            Ok(ipv4) => ip = ipv4,
            Err(_) => {
                eprintln!("Failed to retrieve IPv4 address.");
                return;
            }
        }
    }

    println!("Retrieved IP: {}", ip);

    let get_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type={}",
        config.account.zone_id,
        config.dns.record,
        config.dns.r#type.as_str()
    );

    let cloudflare_response = client
        .get(&get_url)
        .header("Authorization", "Bearer ".to_owned() + &config.account.auth_key)
        .send()
        .await;

    match cloudflare_response {
        Ok(get_response) => {
            let json: serde_json::Value = get_response.json().await.expect("Failed to parse JSON");

            if let Some(record) = json["result"].get(0) {
                if let Some(record_id) = record["id"].as_str() {
                    println!("Get RecordID: {}", record_id);
                    let data = Data {
                        r#type: config.dns.r#type.as_str().to_string(),
                        name: config.dns.record.clone(),
                        content: ip.trim().to_string(),
                        ttl: json["result"][0]["ttl"].as_i64().unwrap() as i32,
                        proxied: config.dns.proxied,
                    };

                    let put_url = format!(
                        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                        config.account.zone_id, record_id
                    );

                    match client
                        .put(&put_url)
                        .header("Authorization", "Bearer ".to_owned() + &config.account.auth_key)
                        .json(&data)
                        .send()
                        .await
                    {
                        Ok(put_response) => {
                            if put_response.status().is_success() {
                                println!("DNS record updated successfully.");
                            } else {
                                eprintln!("Failed to update DNS record.");
                            }
                        }
                        Err(e) => {
                            eprintln!("PUT request error: {}", e);
                        }
                    }
                } else {
                    eprintln!("Record ID not found in the response.");
                }
            } else {
                println!("No DNS records found. Creating a new record...");

                let data = Data {
                    r#type: config.dns.r#type.as_str().to_string(),
                    name: config.dns.record.clone(),
                    content: ip.trim().to_string(),
                    ttl: 120,
                    proxied: config.dns.proxied,
                };

                let post_url = format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                    config.account.zone_id
                );

                match client
                    .post(&post_url)
                    .header("Authorization", "Bearer ".to_owned() + &config.account.auth_key)
                    .json(&data)
                    .send()
                    .await
                {
                    Ok(post_response) => {
                        if post_response.status().is_success() {
                            println!("DNS record created successfully.");
                        } else {
                            eprintln!("Failed to create DNS record.");
                        }
                    }
                    Err(e) => {
                        eprintln!("POST request error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("GET request error: {}", e);
        }
    }

    println!("End");
}
