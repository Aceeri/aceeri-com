extern crate serde;
extern crate serde_json;

use std::fs::{self, File, ReadDir, DirEntry};
use std::io::Read;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use self::serde_json::value::{self, Value};

#[derive(Debug)]
pub struct Data {
	pub value: Result<Value, serde_json::error::Error>
}

impl Data {
	pub fn from_file(path: String) -> Data {
		let data_str = "{".to_owned() + &Data::get_string(path) + "}";

		Data {
			value: serde_json::from_str(&data_str)
		}
	}

	pub fn from_dir(path: String) -> Data {
		let mut data_str = Data::dir_string(path);

		Data {
			value: serde_json::from_str(&data_str)
		}
	}

	pub fn dir_string(path: String) -> String {
		let mut result = r"{".to_owned();

		if fs::metadata(&path).unwrap().is_dir() {
			for (index, entry) in fs::read_dir(&path).unwrap().enumerate() {
				let entry = entry.unwrap();
				let entry_path = entry.path();
				let string_path = entry_path.clone().into_os_string().into_string().unwrap();

				let meta = fs::metadata(&entry_path).unwrap();
				
				let name = Path::new(&entry_path).file_stem().unwrap().to_str().unwrap();

				if index > 0 {
					result = result + ",";
				}

				if meta.is_file() {
					result = result + " \"" + &name + "\" : {" + &Data::get_string(string_path) + " }";
				} else if meta.is_dir() {
					result = result + " \"" + &name + "\" : " + &Data::dir_string(string_path);
				}
			}
		}

		result = result + r"}";
		result
	}

	pub fn get_value(path: String) -> Value {
		let mut reader = File::open(path).unwrap();
		serde_json::from_reader(reader).unwrap()
	}

	pub fn get_string(path: String) -> String {
		let mut buffer = "".to_owned();
		let mut reader = File::open(path).unwrap();
		reader.read_to_string(&mut buffer).unwrap();
		buffer
	}

	pub fn combine(v1: &BTreeMap<String, Value>, v2: &BTreeMap<String, Value>) -> BTreeMap<String, Value> {
		let mut map = BTreeMap::new();

		for (k, v) in v1.iter() {
			map.insert(k.clone().to_owned(), v.clone());
		}

		for (k, v) in v2.iter() {
			map.insert(k.clone().to_owned(), v.clone());
		}
		
		map
	}
}