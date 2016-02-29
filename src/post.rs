extern crate regex;
extern crate serde;
extern crate serde_json;

use std::fs::{self, File, ReadDir, DirEntry};
use std::io::Read;
use std::collections::BTreeMap;
use std::path::PathBuf;

use self::regex::Regex;

use self::serde_json::value::{self, Value};

#[derive(Debug)]
pub struct Post {
	map: BTreeMap<String, Value>
}

impl Post {
	pub fn from_dir(path: PathBuf) -> Vec<Post> {
		let mut posts = Vec::new();

		if fs::metadata(&path).unwrap().is_dir() {
			for entry in fs::read_dir(&path).unwrap() {
				let entry = entry.unwrap().path();
				let meta = fs::metadata(&entry).unwrap();
				if meta.is_file() {
					let string = entry.into_os_string().into_string().unwrap();
					posts.push(Post::from_file(string));
				} else if meta.is_dir() {
					posts.append(&mut Post::from_dir(entry));
				}
			}
		}

		posts
	}

	pub fn from_file(path: String) -> Post {
		let mut file = File::open(&path).unwrap();
		let mut buffer = "".to_owned();
		file.read_to_string(&mut buffer);

		let mut map = BTreeMap::new();

		let re = Regex::new(r#"\s*['"](.+)['"]\s*:\s*['"](.+)['"]\s*"#).unwrap();

		for entry in re.captures_iter(&buffer) {
			let key = path.clone() + "." + entry.at(1).unwrap_or("");
			let value = entry.at(2).unwrap_or("Not found");

			map.insert(key, value::to_value(value));
		}

		Post {
			map: map
		}
	}

	pub fn data(posts: Vec<Post>) -> BTreeMap<String, Value> {
		let mut result = BTreeMap::new();

		for post in posts {
			for (key, value) in post.map.iter() {
				result.insert(key.clone(), value.clone());
			}
		}

		result
	}
}