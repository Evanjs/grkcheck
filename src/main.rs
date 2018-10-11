#![feature(alloc_system)]

extern crate reqwest;
extern crate kuchiki;
extern crate html5ever;
extern crate semver;
extern crate linuxver;

use semver::Version;

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
    let formatted_text = format!("{}", text);
    return formatted_text;
}

fn eval_css_version_selector(text: &str, css_selector: &str) -> String {
    let document = kuchiki::parse_html().one(text);
    let latest_version_string = eval_css_selector(text, css_selector);
    let mut latest_version = Version::parse(&latest_version_string).unwrap();
    for css_match in document.select(css_selector).unwrap() {
        let as_node = css_match.as_node();
        let text_node = as_node.first_child().unwrap();
        let text = text_node.as_text().unwrap().borrow();
        let version = Version::parse(&text).unwrap();
        if version > latest_version {
            latest_version = version;
        }
    }
    return latest_version.to_string();
}

fn get_version(url: &str, css_selector: &str) -> String {
    let result = get_request(url);
    let css_result = eval_css_selector(&result, css_selector);
    return css_result;
}

fn get_versions(url: &str, css_selector: &str) -> String {
    let result = get_request(url);
    let css_result = eval_css_version_selector(&result, css_selector);
    return css_result;
}

fn main() {
    // TODO: Clean up parallelization
    // TODO: more robust version detection.  Latest version isn't always the first on the page
    let gentoo = thread::spawn(|| {
       let version= get_versions(
            "https://packages.gentoo.org/packages/sys-kernel/gentoo-sources",
            "td.kk-version.kk-cell-sep-right > strong > a");
        println!("Gentoo: {}", version);
    });

    // TODO: more robust version detection.  stable (in the version table) isn't always the latest version, which might be shown under #latest_link, etc.
    let vanilla = thread::spawn(|| {
        let version = get_version("https://www.kernel.org", "#latest_link > a");
        println!("Vanilla: {}", version);
    });

    let local = thread::spawn(|| {
        let local_kernel = linuxver::version().unwrap();
        println!("Currently running kernel version: {}.{}.{}", local_kernel.major, local_kernel.minor, local_kernel.patch );
    });

    // #[warn(unused_must_use)] on by default
    let _ = vanilla.join();
    let _ = gentoo.join();
    let _ = local.join();

}
