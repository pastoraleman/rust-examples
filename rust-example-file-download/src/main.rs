extern crate hyper;

use hyper::Client;

use std::fs::File;
use std::io::{Read, Write};

fn main() {

    let client = Client::new();

    let mut res = client.get("https://www.wikipedia.org/portal/wikipedia.org/assets/img/Wikipedia-logo-v2@2x.png").send().unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut my_file = match File::create("logo.png") {
        Ok(buf) => buf,
        Err(e) => panic!(e)
    };

    let mut my_buf: Vec<u8> = Vec::new();

    &res.read_to_end(&mut my_buf);

    my_file.write(&my_buf).unwrap();
}
