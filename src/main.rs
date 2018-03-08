extern crate reqwest;
extern crate kuchiki;
extern crate html5ever;
extern crate linuxver;

use std::thread;
use kuchiki::traits::*;
use std::string::String;

fn get_request(s: &str) -> String {
    let res = reqwest::get(s).unwrap().text();
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

fn get_version(url: &str, css_selector: &str) -> String {
    let result = get_request(url);
    let css_result = eval_css_selector(&result, css_selector);
    return css_result;
}

fn main() {
    // TODO: Clean up parallelization
    let vanilla = thread::spawn(|| {
       let version = get_version(
            "https://packages.gentoo.org/packages/sys-kernel/gentoo-sources",
            "body > div.container > div > div > div > div.col-md-9\
        > div:nth-child(1) > div.table-responsive > table > tbody\
        > tr:nth-child(1) > td.kk-version.kk-cell-sep-right > strong > a");
        println!("Vanilla: {}", version);
    });

    let gentoo = thread::spawn(|| {
        let version = get_version("https://www.kernel.org", "#latest_link > a");
        println!("Gentoo: {}", version);
    });

    let local = thread::spawn(|| {
        let local_kernel = linuxver::version().unwrap();
        println!("Currently running kernel version: {}.{}.{}", local_kernel.major, local_kernel.minor, local_kernel.patch );
    });

    vanilla.join();
    gentoo.join();
    local.join();

}
