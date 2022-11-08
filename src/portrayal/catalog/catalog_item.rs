use super::Description;

pub trait CatalogItem {
    fn id(&self) -> &str;
    fn descriptions(&self) -> &[Description];
}
