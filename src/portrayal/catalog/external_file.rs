use super::{CatalogItem, FileFormat, FileType};

pub trait ExternalFile: CatalogItem {
    fn file_name(&self) -> &str;
    fn file_type(&self) -> FileType;
    fn file_format(&self) -> FileFormat;
}
