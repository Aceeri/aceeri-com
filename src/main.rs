extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

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
  let mut ip = "".to_string();
  let mut file = File::open("../host.txt").unwrap();
  file.read_to_string(&mut ip);

  let host: &str = &ip;
  println!("HOST: {:?}", host);

  let mut router = Router::new();
  router
    .get("/", Static::new(Path::new("res/index.html")));

  let mut mount = Mount::new();
  mount
    .mount("/", router)
    .mount("/c/", |req: &mut Request| {
      let res = Static::new(Path::new("res/")).handle(req);
      println!("REQ: {:?} => RES: {:?}", req, res);
      res
    });

  Iron::new(mount).http(host).unwrap();
}