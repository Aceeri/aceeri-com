extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate regex;

use regex::Regex;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use router::Router;
use router::Params;

use mount::Mount;

use staticfile::Static;

fn main() {
  let mut content = "".to_string();
  let mut file = File::open("../host.txt").unwrap();
  file.read_to_string(&mut content);

  let re = Regex::new(r"(.+):(\d+)").unwrap();
  let captures = re.captures(&content).unwrap();

  let host = captures.at(0).unwrap();
  println!("HOST: {:?}", host);

  let mut router = Router::new();

  let mut mount = Mount::new();
  mount
    .mount("/", Static::new(Path::new("pages/")))
    .mount("/c/", Static::new(Path::new("res/")));

  Iron::new(mount).http(host).unwrap();
}