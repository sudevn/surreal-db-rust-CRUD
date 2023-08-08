#![allow(unused)]

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value, self};
use surrealdb::Surreal;
use std::collections::BTreeMap;
use anyhow::{anyhow, Result};



#[derive(Debug, Serialize)]
struct Client<'a> {
    name: &'a str,
    tag: &'a str,
    datecreated: &'a str,
}
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}


#[tokio::main]
pub async fn main() -> Result<()> {
    //--connect
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("my_ns").use_db("my_db").await?;
    println!("\x1b[93m1. Connected\x1b[0m");

    // --Create
    let created: Record = db.create("client").content(Client {name: "Kattera", tag: "Arch", datecreated: "08-08-2023"}).await?;
    println!("");
    println!("\x1b[93m2. Created\x1b[0m");
    println!("");

    // --Read (or advanced sql)
    let clents: surrealdb::Response = db.query("SELECT * from client ").await?;
    print!("{:?}", clents);
    println!("");
    println!("\x1b[93m3. Read\x1b[0m");
    println!("");

    // --Upadte
    let update: Record = db.update(("client","Kattera")).content(Client {name: "Alpha", tag: "Rubber", datecreated: "08-08-2023"}).await?;
    print!("{:?}", update);
    let clents: surrealdb::Response = db.query("SELECT * from client ").await?;
    print!("{:?}", clents);
    println!("");
    println!("\x1b[93m4. Updated\x1b[0m");
    println!("");
    
    // --Remove
    let delete: Vec<Record> = db.delete(("client")).await?;
    let clents: surrealdb::Response = db.query("SELECT * from client ").await?;
    print!("{:?}", clents);
    println!("");
    println!("\x1b[93m5. Deleted\x1b[0m");
    println!("");

    println!("\x1b[92mSuccessfull CRUD\x1b[0m 🙌🎉🙌🎉"); 
    Ok(())
}