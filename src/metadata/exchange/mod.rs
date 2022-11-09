mod dataset_discovery_metadata;
pub use dataset_discovery_metadata::DatasetDiscoveryMetadata;

mod data_format;
pub use data_format::DataFormat;

mod exchange_catalog;
pub use exchange_catalog::ExchangeCatalog;

const EXCHANGE_CATALOG: &str = "S100_ExchangeCatalogue";
const DATA_DISCOVERY_METADATA: &str = "S100_DatasetDiscoveryMetadata";
