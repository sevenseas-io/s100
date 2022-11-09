use std::{env, fs};

use s100::metadata::exchange::ExchangeCatalog;

#[test]
fn read_exchange_catalogs() {
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("data")
        .join("S-101-Test-Datasets");

    read_exchange_catalogs_dir(current_dir);
}

fn read_exchange_catalogs_dir(current_dir: std::path::PathBuf) {
    for entry in fs::read_dir(current_dir).unwrap() {
        let path = entry.unwrap().path();

        let metadata = fs::metadata(&path).unwrap();

        if metadata.is_dir() && path.file_name().unwrap() != "archive" {
            read_exchange_catalogs_dir(path)
        } else {
            if metadata.is_file() && path.file_name().unwrap() == "CATALOG.XML" {
                match ExchangeCatalog::open(&path) {
                    Ok(_) => {}
                    Err(err) => panic!(
                        "could not read exchange catalog at {}: {}",
                        path.to_str().unwrap(),
                        err
                    ),
                }
            }
        }
    }
}
