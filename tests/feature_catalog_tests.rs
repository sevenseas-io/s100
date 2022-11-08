use s100::feature::catalog::FeatureCatalog;

#[test]
#[allow(non_snake_case)]
fn read_S_101_FC_main() {
    let path1 = std::env::current_dir().unwrap();
    println!("The current directory is {}", path1.display());
    let path = "tests/data/S-101_Portrayal-Catalogue/FeatureCatalog.xml";
    match FeatureCatalog::open(path) {
        Ok(catalog) => {
            assert_eq!(220, catalog.simple_attributes().len());
        }
        Err(err) => panic!("open returned an unexpected error: {}", err),
    }
}
