extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use router::Router;
use router::Params;

use mount::Mount;

use staticfile::Static;

fn main() {
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

  Iron::new(mount).http("localhost:3000").unwrap();
}