
extern crate serde_json;
extern crate handlebars;
extern crate regex;

use self::serde_json::value::Value;

use self::regex::Regex;

use std::fs::{self, DirBuilder, File};
use std::path::{Path, PathBuf, Component};

use self::handlebars::{Handlebars, Context};
use super::data::Data;

pub struct Render {
  handlebars: Handlebars,
  data: Value,
  out: PathBuf // folder to output to
}

impl Render {
  pub fn new(data: Data) -> Render {
    Render {
      handlebars: Handlebars::new(),
      data: data.value.unwrap(),
      out: PathBuf::from("rendered/")
    }
  }

  pub fn render(&mut self, path: String) {
    self.render_dir(path);
  }

  pub fn render_dir(&mut self, path: String) {
    if fs::metadata(&path).unwrap().is_dir() {
      for (index, entry) in fs::read_dir(&path).unwrap().enumerate() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let string_path = entry_path.clone().into_os_string().into_string().unwrap();
        let meta = fs::metadata(&entry_path).unwrap();
        let name = entry.file_name().to_owned().into_string().unwrap();

        if meta.is_file() {
          match Data::from_file(string_path).value {
            Ok(file) => {
              let template = file.as_object().unwrap().get("template").unwrap().as_string().unwrap();

              if self.handlebars.get_template(template) == None {
                self.handlebars.register_template_file(template, &Path::new(template)).ok().unwrap();
              }

              let mut writer_path = PathBuf::new();
              writer_path.push(self.out.as_path());

              let mut result = PathBuf::new();
              for (index, component) in entry_path.iter().enumerate() {
                if index > 0 {
                  result.push(component);
                }
              }
              writer_path.push(result);
              writer_path.set_extension("html");

              let mut writer_dir = writer_path.clone();
              writer_dir.pop();

              DirBuilder::new().recursive(true).create(writer_dir);

              let mut writer = File::create(&writer_path).unwrap();
              let context = Context::wraps(&self.data);

              let mut page = self.handlebars.render(template, &file).unwrap();

              let relt = Regex::new(r"&lt;").unwrap();
              let regt = Regex::new(r"&gt;").unwrap();


              page = relt.replace_all(&page, "<");
              page = regt.replace_all(&page, ">");

              let template_name = &writer_path.into_os_string().into_string().unwrap();

              self.handlebars.register_template_string(template_name, page).ok().unwrap();
              self.handlebars.renderw(template_name, &context, &mut writer);

            },
            Err(err) => {
              println!("ERR: {:?}", err);
            }
          }
          
        } else if meta.is_dir() {
          self.render_dir(string_path);
        }
      }
    }
  }
}