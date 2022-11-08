mod attribute_value_type;
pub use attribute_value_type::AttributeValueType;

mod definition_reference;
pub use definition_reference::DefinitionReference;

mod feature_catalog;
pub use feature_catalog::FeatureCatalog;

mod item;
pub use item::Item;

mod listed_value;
pub use listed_value::ListedValue;

mod simple_attribute;
pub use simple_attribute::SimpleAttribute;

const DEFINITION_REFERENCE: &str = "definitionReference";
const FEATURE_CATALOG: &str = "S100_FC_FeatureCatalogue";
const LISTED_VALUE: &str = "listedValue";
const SIMPLE_ATTRIBUTE: &str = "S100_FC_SimpleAttribute";

const XML_REF: &str = "ref";
