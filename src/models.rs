/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Poems {
    // pub id: i32,
    pub _id: Map<String, Value>,
    pub title: String,
    pub dynasty: Option<String>,
    pub writer: Option<String>,
    pub poemtype: Option<Vec<String>>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
    // pub poemtype: Vec<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ResPoems {
    pub id: i32,
    pub _id: Option<String>,
    pub title: String,
    pub dynasty: Option<String>,
    pub writer: Option<String>,
    pub poemtype: Option<Vec<String>>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
    // pub poemtype: Vec<String>,
}

impl From<Poems> for NewPoem {
    fn from(item: Poems) -> Self {
        let Poems {
            _id,
            title,
            dynasty,
            writer,
            poemtype,
            content,
            remark,
            translation,
            shangxi,
        } = item;
        let id: String = match _id.get("$oid").unwrap().to_owned() {
            Value::String(x) => x,
            _ => String::new(),
        };
        NewPoem {
            _id: Some(id),
            title,
            dynasty,
            writer,
            poemtype,
            content,
            remark,
            translation,
            shangxi,
        }
    }
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "poems"]
pub struct NewPoem {
    pub _id: Option<String>,
    pub title: String,
    pub dynasty: Option<String>,
    pub writer: Option<String>,
    pub poemtype: Option<Vec<String>>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
}
