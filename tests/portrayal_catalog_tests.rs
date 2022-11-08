use s100::portrayal::catalog::PortrayalCatalog;

#[test]
#[allow(non_snake_case)]
fn read_S_101_PC_main() {
    let path1 = std::env::current_dir().unwrap();
    println!("The current directory is {}", path1.display());
    let path = "tests/data/S-101_Portrayal-Catalogue/PortrayalCatalog/portrayal_catalogue.xml";
    match PortrayalCatalog::open(path) {
        Ok(catalog) => {
            assert_eq!(catalog.alert_catalog().is_some(), true);
            assert_eq!(catalog.area_fills().len(), 26);
            assert_eq!(catalog.color_profiles().len(), 1);
            assert_eq!(catalog.display_modes().len(), 3);
            assert_eq!(catalog.display_planes().len(), 2);
            assert_eq!(catalog.fonts().len(), 0);
            assert_eq!(catalog.foundation_mode().len(), 19);
            assert_eq!(catalog.line_styles().len(), 64);
            assert_eq!(catalog.pixmaps().len(), 0);
            assert_eq!(catalog.rules().len(), 211);
            assert_eq!(catalog.style_sheets().len(), 3);
            assert_eq!(catalog.symbols().len(), 619);
            assert_eq!(catalog.viewing_groups().len(), 147);
            assert_eq!(catalog.viewing_group_layers().len(), 27);
        }
        Err(err) => panic!("open returned an unexpected error: {}", err),
    }
}
