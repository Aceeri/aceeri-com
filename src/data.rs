extern crate serde;
extern crate serde_json;

use std::fs::{self, File, ReadDir, DirEntry};
use std::io::Read;
use std::collections::BTreeMap;
use std::path::PathBuf;

use self::serde_json::value::{self, Value};

#[derive(Debug)]
pub struct Data {
	pub value: Result<Value, serde_json::error::Error>
}

impl Data {
	pub fn from(path: String) -> Data {
		let mut data_str = Data::dir_string(path);

		println!("DATA: {:?}", data_str);

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
				
				let name = entry.file_name().to_owned().into_string().unwrap();

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
}