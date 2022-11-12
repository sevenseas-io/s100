use libxml::parser::Parser;
use std::path::Path;

use super::{
    AlertCatalog, AreaFill, ColorProfile, DisplayMode, DisplayPlane, Font, LineStyle, Parameter,
    Pixmap, RuleFile, StyleSheet, Symbol, ViewingGroup, ViewingGroupLayer, ALERT_CATALOG,
    PORTRAYAL_CATALOG,
};
use crate::{Result, S100Error};

// Shcema defined here: https://schemas.s100dev.net/schemas/S100/5.0.0/S100PC/20220705/S100PortrayalCatalog.xsd
#[derive(Debug)]
pub struct PortrayalCatalog {
    alert_catalog: Option<AlertCatalog>,
    area_fills: Vec<AreaFill>,
    color_profiles: Vec<ColorProfile>,
    context: Vec<Parameter>,
    display_modes: Vec<DisplayMode>,
    display_planes: Vec<DisplayPlane>,
    fonts: Vec<Font>,
    foundation_mode: Vec<String>,
    line_styles: Vec<LineStyle>,
    pixmaps: Vec<Pixmap>,
    rules: Vec<RuleFile>,
    style_sheets: Vec<StyleSheet>,
    symbols: Vec<Symbol>,
    viewing_groups: Vec<ViewingGroup>,
    viewing_group_layers: Vec<ViewingGroupLayer>,
}

const AREA_FILLS: &str = "areaFills";
const COLOR_PROFILES: &str = "colorProfiles";
const CONTEXT: &str = "context";
const DISPLAY_MODES: &str = "displayModes";
const DISPLAY_PLANES: &str = "displayPlanes";
const FONTS: &str = "fonts";
const FOUNDATION_MODE: &str = "foundationMode";
const LINE_STYLES: &str = "lineStyles";
const PIXMAPS: &str = "pixmaps";
const RULES: &str = "rules";
const STYLE_SHEETS: &str = "styleSheets";
const SYMBOLS: &str = "symbols";
const VIEWING_GROUPS: &str = "viewingGroups";
const VIEWING_GROUP_LAYERS: &str = "viewingGroupLayers";

impl PortrayalCatalog {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<PortrayalCatalog> {
        let parser = Parser::default();
        let filename_result = path.as_ref().to_str();
        match filename_result {
            Some(filename) => {
                let document = parser.parse_file(filename)?;

                if let Some(root) = document.get_root_element() {
                    let mut portrayal_catalog = PortrayalCatalog {
                        alert_catalog: None,
                        area_fills: Vec::new(),
                        color_profiles: Vec::new(),
                        context: Vec::new(),
                        display_planes: Vec::new(),
                        display_modes: Vec::new(),
                        fonts: Vec::new(),
                        foundation_mode: Vec::new(),
                        line_styles: Vec::new(),
                        pixmaps: Vec::new(),
                        rules: Vec::new(),
                        style_sheets: Vec::new(),
                        symbols: Vec::new(),
                        viewing_groups: Vec::new(),
                        viewing_group_layers: Vec::new(),
                    };

                    if root.get_name() == PORTRAYAL_CATALOG {
                        for node in root.get_child_elements() {
                            match node.get_name().as_str() {
                                ALERT_CATALOG => match AlertCatalog::parse(node) {
                                    Ok(alert_catalog) => {
                                        portrayal_catalog.alert_catalog = Some(alert_catalog)
                                    }
                                    Err(e) => return Err(e),
                                },
                                AREA_FILLS => {
                                    let area_fills_node = node.get_child_elements();
                                    for area_fill_node in area_fills_node {
                                        match AreaFill::parse(area_fill_node) {
                                            Ok(area_fill) => {
                                                portrayal_catalog.area_fills.push(area_fill)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                COLOR_PROFILES => {
                                    let color_profiles_node = node.get_child_elements();
                                    for color_profile_node in color_profiles_node {
                                        match ColorProfile::parse(color_profile_node) {
                                            Ok(color_profile) => {
                                                portrayal_catalog.color_profiles.push(color_profile)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                CONTEXT => {
                                    let parameters_node = node.get_child_elements();
                                    for parameter_node in parameters_node {
                                        match Parameter::parse(parameter_node) {
                                            Ok(parameter) => {
                                                portrayal_catalog.context.push(parameter)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                DISPLAY_MODES => {
                                    let display_modes_node = node.get_child_elements();
                                    for display_mode_node in display_modes_node {
                                        match DisplayMode::parse(display_mode_node) {
                                            Ok(display_mode) => {
                                                portrayal_catalog.display_modes.push(display_mode)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                DISPLAY_PLANES => {
                                    let display_planes_node = node.get_child_elements();
                                    for display_plane_node in display_planes_node {
                                        match DisplayPlane::parse(display_plane_node) {
                                            Ok(display_plane) => {
                                                portrayal_catalog.display_planes.push(display_plane)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                FONTS => {
                                    let fonts_node = node.get_child_elements();
                                    for font_node in fonts_node {
                                        match Font::parse(font_node) {
                                            Ok(font) => portrayal_catalog.fonts.push(font),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                FOUNDATION_MODE => {
                                    let viewing_group_nodes = node.get_child_elements();
                                    for viewing_group in viewing_group_nodes {
                                        portrayal_catalog
                                            .foundation_mode
                                            .push(viewing_group.get_content())
                                    }
                                }
                                LINE_STYLES => {
                                    let line_styles_node = node.get_child_elements();
                                    for line_style_node in line_styles_node {
                                        match LineStyle::parse(line_style_node) {
                                            Ok(line_style) => {
                                                portrayal_catalog.line_styles.push(line_style)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                PIXMAPS => {
                                    let pixmaps_node = node.get_child_elements();
                                    for pixmap_node in pixmaps_node {
                                        match Pixmap::parse(pixmap_node) {
                                            Ok(pixmap) => portrayal_catalog.pixmaps.push(pixmap),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                STYLE_SHEETS => {
                                    let style_sheets_node = node.get_child_elements();
                                    for style_sheet_node in style_sheets_node {
                                        match StyleSheet::parse(style_sheet_node) {
                                            Ok(style_sheet) => {
                                                portrayal_catalog.style_sheets.push(style_sheet)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                SYMBOLS => {
                                    let symbols_node = node.get_child_elements();
                                    for symbol_node in symbols_node {
                                        match Symbol::parse(symbol_node) {
                                            Ok(symbol) => portrayal_catalog.symbols.push(symbol),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                RULES => {
                                    let rules_node = node.get_child_elements();
                                    for rule_node in rules_node {
                                        match RuleFile::parse(rule_node) {
                                            Ok(rule) => portrayal_catalog.rules.push(rule),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                VIEWING_GROUPS => {
                                    let viewing_groups_node = node.get_child_elements();
                                    for viewing_group_node in viewing_groups_node {
                                        match ViewingGroup::parse(viewing_group_node) {
                                            Ok(viewing_group) => {
                                                portrayal_catalog.viewing_groups.push(viewing_group)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                VIEWING_GROUP_LAYERS => {
                                    let viewing_group_layers_node = node.get_child_elements();
                                    for viewing_group_layer_node in viewing_group_layers_node {
                                        match ViewingGroupLayer::parse(viewing_group_layer_node) {
                                            Ok(viewing_group_layer) => portrayal_catalog
                                                .viewing_group_layers
                                                .push(viewing_group_layer),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                _ => return S100Error::invalid_child(node),
                            }
                        }

                        Ok(portrayal_catalog)
                    } else {
                        Err(S100Error::Parse(format!(
                            "Root node is not {}, found '{} instead",
                            PORTRAYAL_CATALOG,
                            root.get_name(),
                        )))
                    }
                } else {
                    Err(S100Error::Parse("Root node does not exist".to_string()))
                }
            }
            None => Err(S100Error::Parse("path is empty".to_string())),
        }
    }

    pub fn alert_catalog(&self) -> Option<&AlertCatalog> {
        match &self.alert_catalog {
            Some(val) => Some(val),
            None => None,
        }
    }

    pub fn area_fills(&self) -> &[AreaFill] {
        &self.area_fills
    }

    pub fn color_profiles(&self) -> &[ColorProfile] {
        &self.color_profiles
    }

    pub fn context(&self) -> &[Parameter] {
        &self.context
    }

    pub fn fonts(&self) -> &[Font] {
        &self.fonts
    }

    pub fn foundation_mode(&self) -> Vec<&str> {
        self.foundation_mode.iter().map(|s| s.as_str()).collect()
    }

    pub fn display_modes(&self) -> &[DisplayMode] {
        &self.display_modes
    }

    pub fn display_planes(&self) -> &[DisplayPlane] {
        &self.display_planes
    }

    pub fn line_styles(&self) -> &[LineStyle] {
        &self.line_styles
    }

    pub fn pixmaps(&self) -> &[Pixmap] {
        &self.pixmaps
    }

    pub fn rules(&self) -> &[RuleFile] {
        &self.rules
    }

    pub fn style_sheets(&self) -> &[StyleSheet] {
        &self.style_sheets
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    pub fn viewing_groups(&self) -> &[ViewingGroup] {
        &self.viewing_groups
    }

    pub fn viewing_group_layers(&self) -> &[ViewingGroupLayer] {
        &self.viewing_group_layers
    }
}
