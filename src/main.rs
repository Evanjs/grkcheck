extern crate reqwest;

use std::error::Error;
use std::string::String;
use reqwest::Client;

fn get_request(client: &Client, s: &str) -> Result<String, Box<Error>> {
    let text = client.get(s).send()?.text();
    Ok(format!("body = {:?}", text).into())
}

fn get_vanilla(client: &Client) -> String {
    let u: &str = &String::from("https://www.kernel.org");
    let result = get_request(client, u).unwrap();
    return result;
}

fn get_gentoo(client: &Client) -> String {
    let u: &str = &String::from("https://packages.gentoo.org/packages/sys-kernel/gentoo-sources");
    let result = get_request(client, u).unwrap();
    return result;
}

fn main() {
    let client = reqwest::Client::new();
    let vanilla = get_vanilla(&client);
    let gentoo = get_gentoo(&client);

    println!("{}", vanilla);
    println!("{}", gentoo);
}
