
extern crate backend;

extern crate iron;
extern crate handlebars;
extern crate mount;
extern crate staticfile;
extern crate regex;

use regex::Regex;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use handlebars::Handlebars;

use backend::post::Post;
use backend::data::Data;

fn main() {
  let host = match get_host("../host.txt") {
    Ok(content) => content,
    Err(err) => {
      println!("ERR: {:?}", err);
      "127.0.0.1:3000".to_owned()
    }
  };
  println!("HOST: {:?}", host);

  let mut handlebars = Handlebars::new();
  let mut data = Data::from("data".to_owned()).value.unwrap();
  println!("DATA: {:?}", data);

  //let data = Post::data(Post::from_dir(PathBuf::from("data/")));
  //println!("POSTS: {:?}", data);

  handlebars.register_template_file("index", &Path::new("templates/index.hbs")).ok().unwrap();
  let index = handlebars.render("index", &data.as_object().unwrap());
  println!("INDEX: {:?}", index);

  let mut mount = Mount::new();
  mount
    .mount("/", Static::new(Path::new("pages/")))
    .mount("/c/", Static::new(Path::new("res/")))
    .mount("/p/", Static::new(Path::new("rendered/posts/")));

  let ip: &str = &host; // cannot pass coerced type with bounds
  Iron::new(mount).http(&ip).unwrap();
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
