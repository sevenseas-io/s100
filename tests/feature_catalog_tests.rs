use std::{env, fs};

use s100::feature::catalog::FeatureCatalog;

#[test]
#[allow(non_snake_case)]
fn read_S_101_FC_main() {
    let path = env::current_dir()
        .unwrap()
        .join("tests")
        .join("data")
        .join("S-101_Portrayal-Catalogue")
        .join("FeatureCatalog.xml");

    match FeatureCatalog::open(path) {
        Ok(catalog) => {
            assert_eq!(220, catalog.simple_attributes().len());
        }
        Err(err) => panic!("open returned an unexpected error: {}", err),
    }
}
