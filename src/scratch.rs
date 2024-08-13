//! The actual scratch backend that generates the final `.sb3`

pub mod expr;
pub mod cond;
pub mod statement;
pub mod assembler;

pub use expr::*;
pub use cond::*;
pub use statement::*;
pub use assembler::*;

use std::{fs::File, io::Write, path::Path};
use json::{object, JsonValue};
use zip::{write::SimpleFileOptions, ZipWriter};

#[inline]
fn block_idx_to_id(idx: usize) -> String {
    format!("block_idx: {idx}")
}

/// Takes the json output of `assemble` and writes it to a zip file of the path specified
pub fn write_to_zip(path: impl AsRef<Path>, json: JsonValue) -> Result<(), std::io::Error> {
    let mut zip = ZipWriter::new(File::create(path)?);

    // write the json
    zip.start_file("project.json", SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated))?;
    zip.write_all(json.to_string().as_bytes())?;

    // write the required svg asset
    zip.start_file("cd21514d0531fdffb22204e0ec5ed84a.svg", SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated))?;
    zip.write_all(include_str!("asset.svg").as_bytes())?;

    // finish
    zip.finish()?;
    Ok(())
}

/// Takes the json output of `assemble` and makes a list visible for the `console`
pub fn set_console(list_ident: &str, mut json: JsonValue) -> JsonValue {
    json["monitors"].push(object! {
        id: list_ident,
        mode: "list",
        opcode: "data_listcontents",
        params: {
            List: list_ident,
        },
        spriteName: null,
        value: [],
        width: 480,
        heigh: 360,
        x: 0,
        y: 0,
        visible: true,
    }).unwrap();
    json
}
