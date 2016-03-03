
extern crate backend;

extern crate iron;
extern crate hyper;
extern crate handlebars;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::net::IpAddr;
use std::collections::HashSet;

use hyper::header::{Headers, CacheControl, CacheDirective};
use iron::{Iron, Handler, Response, Request, IronResult, IronError, Url, status};
use iron::middleware::Chain;
use mount::Mount;
use router::Router;
use staticfile::Static;

use serde_json::value::Value;
use regex::Regex;

use backend::data::Data;
use backend::render::Render;

fn main() {
  let host = match get_host("../host.txt") {
    Ok(content) => content,
    Err(err) => {
      println!("ERR: {:?}", err);
      "127.0.0.1:3000".to_owned()
    }
  };
  println!("HOST: {:?}", host);

  let data = Data::from_dir("data".to_owned());

  let mut render = Render::new(data);
  render.render("templates/".to_owned());
  render.render("data/".to_owned());

  let mut mount = Mount::new();
  mount
    .mount("/", Static::new(Path::new("rendered/sidebar")))
    .mount("/c/", Static::new(Path::new("res/")))
    .mount("/p/", Static::new(Path::new("rendered/posts")));

  let mut chain = Chain::new(mount);
  chain.link_after(|_: &mut Request, mut res: Response| {
    //res.headers.set(CacheControl(vec![CacheDirective::MaxAge(14400u32)]));
    Ok(res)
  });

  let ip: &str = &host; // cannot pass coerced type with bounds
  Iron::new(chain).http(&ip).unwrap();
}

fn get_host(path: &'static str) -> Result<String, String> {
  let mut file = try!(File::open(path).map_err(|err| err.to_string()));

  let mut content = "".to_owned();
  file.read_to_string(&mut content).ok().unwrap();

  let re = Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?):[0-9][0-9]?[0-9]?[0-9]?[0-9]?").unwrap();
  let captures = re.captures(&content);

  return match captures {
    Some(list) => Ok(list.at(0).unwrap().to_owned()),
    None => Err("IP string could not be found".to_owned())
  };
}
