#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Portalapiaccessstore {
    storeId:     String,
    typeId: String,
    objects: Vec<Object>,
}
#[derive(Deserialize, Debug)]
struct Object {
    organizationId:     String,
    createdBy: String,
    entityId: String,
    id: String,
    createdOn:  i64,
    apiId:  String,
    enabled:    bool,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Portalapiaccessstore, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(file)?;

    // Return the `User`.
    Ok(u)
}


fn main() {
    let u = read_user_from_file("pages.json4").unwrap();
    println!("{:#?}", u);
}