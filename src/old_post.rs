
extern crate serde;
extern crate serde_json;

use self::serde::{ ser, de };
use self::serde::{ Serialize, Serializer, Deserialize, Deserializer };

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Post {
	title: String,
	date: String,
	content: String
}

pub enum PostField {
	Title,
	Date,
	Content
}

impl Post {
	pub fn from_file(path: String) -> Post {
		let mut file = File::open(path).unwrap();
		let mut buffer = "".to_owned();
		file.read_to_string(&mut buffer);

		Post::from_str(buffer)
	}

	pub fn from_str(string: String) -> Post {
		serde_json::from_str::<Post>(&string).unwrap()
	}
}

impl Serialize for Post {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_struct("Post", SerVisitor(self, 0))
    }
}

struct SerVisitor;

impl ser::MapVisitor for SerVisitor {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        self.1 += 1;
        match self.1 {
            1 => serializer.serialize_struct_elt("title", &self.0.title).map(Some),
            2 => serializer.serialize_struct_elt("date", &self.0.date).map(Some),
            3 => serializer.serialize_struct_elt("content", &self.0.content).map(Some),
            _ => Ok(None),
        }
    }

    fn len(&self) -> Option<usize> { Some(3) }
}

impl Deserialize for Post {
	fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        static FIELDS: &'static [&'static str] = &["title", "content"];
        deserializer.deserialize_struct("Post", FIELDS, DeVisitor)
    }
}

struct DeVisitor;

impl de::Visitor for DeVisitor {
	type Value = Post;

	fn visit_map<V: de::MapVisitor>(&mut self, mut visitor: V) -> Result<Post, V::Error> {
		let mut title = None;
		let mut date = None;
        let mut content = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::Title) => title = Some(try!(visitor.visit_value())),
                Some(Field::Date) => title = Some(try!(visitor.visit_value())),
                Some(Field::Content) => content = Some(try!(visitor.visit_value())),
                None => break,
            }
        }

        let title = match title {
            Some(t) => t,
            None => "Title".to_owned(),
        };
        let date = match date {
            Some(d) => d,
            None => "UNKNOWN".to_owned(),
        };
        let content = match content {
            Some(c) => c,
            None => "Content".to_owned(),
        };

        try!(visitor.end());

        Ok(Post {
            title: title,
            date: date,
            content: content,
        })
	}
}

enum Field {
    Title,
    Date,
    Content,
}

impl Deserialize for Field {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize(FieldVisitor)
    }
}

struct FieldVisitor;
impl de::Visitor for FieldVisitor {
    type Value = Field;

    fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<Field, E> {
        match value {
            "title" => Ok(Field::Title),
            "content" => Ok(Field::Content),
            "date" => Ok(Field::Date),
            _ => Err(de::Error::custom("Unexpected value")),
        }
    }
}