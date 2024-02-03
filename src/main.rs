use serde::Serialize;
use serde_json;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

#[derive(Serialize)]
struct SkuCount {
    total: usize,
    sku_list: Vec<String>,
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
}

fn main() -> io::Result<()> {
    let path = Path::new(".");
    let mut sku_list = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if is_dir(&entry) {
            sku_list.push(entry.file_name().into_string().unwrap());
        }
    }

    let folder_info = SkuCount {
        total: sku_list.len(),
        sku_list,
    };

    let json = serde_json::to_string_pretty(&folder_info).unwrap();
    fs::write("count.json", json)?;

    Ok(())
}
