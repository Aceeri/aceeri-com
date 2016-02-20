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
//use iron::status;
//use iron::middleware::Handler;

use router::Router;
//use router::Params;

use mount::Mount;

use staticfile::Static;

fn main() {
  let host = match get_host("../host.txt") {
    Ok(content) => {
      content
    },
    Err(err) => {
      println!("ERR: {:?}", err);
      "localhost:3000".to_owned()
    }
  };
  println!("HOST: {:?}", host);

  let mut router = Router::new();

  let mut mount = Mount::new();
  mount
    .mount("/", Static::new(Path::new("pages/")))
    .mount("/c/", Static::new(Path::new("res/")));
   // .mount("/p/", Static::new(Path::new("pages/"))); // templated pages with project here

  let ip: &str = &host;
  Iron::new(mount).http(&ip).unwrap();
}

fn load() {

}

fn get_host(path: &'static str) -> Result<String, String> {
  let mut file = try!(File::open(path).map_err(|err| err.to_string()));

  let mut content = "".to_owned();
  file.read_to_string(&mut content);

  let re = try!(Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?):[0-9][0-9]?[0-9]?[0-9]?[0-9]?").map_err(|err| err.to_string()));
  let captures = re.captures(&content);

  match captures {
    Some(captured) => {
      return Ok(captured.at(0).unwrap().to_owned())
    },
    None => { }
  };

  return Err("IP string could not be found".to_owned());
}