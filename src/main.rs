extern crate reqwest;
extern crate kuchiki;
extern crate html5ever;
extern crate linuxver;
//extern crate semver;

use kuchiki::traits::*;
use std::string::String;
use reqwest::Client; 
//use semver::Version;

fn get_request(client: &Client, s: &str) -> String {
    let res = client.get(s).send().unwrap().text();
    let body = format!("{}", res.unwrap().to_string());
    return body;
}

fn eval_css_selector(text: &str, css_selector: &str) -> String{
    let doc = kuchiki::parse_html().one(text);
    let css_match = doc.select_first(css_selector).unwrap();
    let as_node = css_match.as_node();
    let text_node = as_node.first_child().unwrap();
    let text = text_node.as_text().unwrap().borrow();
    let formatted_text = format!("{:?}", text);
    return formatted_text;
}

fn get_vanilla(client: &Client) -> String {
    let u: &str = &String::from("https://www.kernel.org");
    let css_selector: &str = &String::from("#latest_link > a");
    let result = get_request(client, u);
    let css_result = eval_css_selector(&result, css_selector);
    return css_result;
}

fn get_gentoo(client: &Client) -> String {
    let u: &str = &String::from("https://packages.gentoo.org/packages/sys-kernel/gentoo-sources");
    let css_selector: &str = &String::from("body > div.container > div > div > div > div.col-md-9 > div:nth-child(1) > div.table-responsive > table > tbody > tr:nth-child(1) > td.kk-version.kk-cell-sep-right > strong > a");
    let result = get_request(client, u);
    let css_result = eval_css_selector(&result, css_selector);
    return css_result;
}

fn main() {
    let client = reqwest::Client::new();
    let vanilla = get_vanilla(&client);
    let gentoo = get_gentoo(&client);
    let local_kernel = linuxver::version().unwrap();

    println!("Vanilla: {}", vanilla);
    println!("Gentoo: {}", gentoo);
    println!("Currently running kernel version: {}.{}.{}", local_kernel.major, local_kernel.minor, local_kernel.patch );

}
