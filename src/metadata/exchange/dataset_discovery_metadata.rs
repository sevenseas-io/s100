use libxml::tree::Node;
use std::str::FromStr;

use super::{DataFormat, DATA_DISCOVERY_METADATA};
use crate::{Error, Result};

const FILE_NAME: &str = "fileName";
const FILE_PATH: &str = "filePath";
const DESCRIPTION: &str = "description";
const DATA_TYPE: &str = "dataType";
const DATA_TYPE_VERSION: &str = "dataTypeVersion";

pub struct DatasetDiscoveryMetadata {
    file_name: String,
    file_path: String,
    description: String,
    data_type: DataFormat,
    data_type_version: String,
}

impl DatasetDiscoveryMetadata {
    pub(super) fn parse(node: Node) -> Result<DatasetDiscoveryMetadata> {
        if node.get_name() != DATA_DISCOVERY_METADATA {
            return Error::invalid_child(node);
        }

        let mut file_name: Option<String> = None;
        let mut file_path: Option<String> = None;
        let mut description: Option<String> = None;
        let mut data_type: Option<DataFormat> = None;
        let mut data_type_version: Option<String> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                FILE_NAME => file_name = Some(child_node.get_content()),
                FILE_PATH => file_path = Some(child_node.get_content()),
                DESCRIPTION => description = Some(child_node.get_content()),
                DATA_TYPE => match DataFormat::from_str(child_node.get_content().as_str()) {
                    Ok(val) => data_type = Some(val),
                    Err(_) => return Error::invalid_value(child_node),
                },
                DATA_TYPE_VERSION => data_type_version = Some(child_node.get_content()),
                _ => {
                    //TODO: return error if we find unrecognized element
                    //return Error::invalid_child(child_node);
                }
            };
        }

        if file_name.is_none() {
            return Error::missing_child(node, FILE_NAME);
        }
        if file_path.is_none() {
            return Error::missing_child(node, FILE_PATH);
        }
        if description.is_none() {
            return Error::missing_child(node, DESCRIPTION);
        }
        if data_type.is_none() {
            return Error::missing_child(node, DATA_TYPE_VERSION);
        }

        Ok(DatasetDiscoveryMetadata {
            file_name: file_name.unwrap(),
            file_path: file_path.unwrap(),
            description: description.unwrap(),
            data_type: data_type.unwrap(),
            data_type_version: data_type_version.unwrap(),
        })
    }

    pub fn file_name(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn data_type(&self) -> DataFormat {
        self.data_type
    }

    pub fn data_type_version(&self) -> &str {
        self.data_type_version.as_str()
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::{DataFormat, DatasetDiscoveryMetadata};

    #[test]
    fn deserialize() {
        let xml = r#"
            <S100XC:S100_DatasetDiscoveryMetadata xmlns:S100XC="http://www.iho.int/s100/xc">
                <S100XC:fileName>101AA00AA1NPOL3.000</S100XC:fileName>
                <S100XC:filePath>101AA00AA1NPOL3</S100XC:filePath>
                <S100XC:description>AA1NPOL3V41</S100XC:description>
                <S100XC:dataProtection>false</S100XC:dataProtection>
                <S100XC:protectionScheme>S100p154.0.0</S100XC:protectionScheme>
                <S100XC:digitalSignatureReference>dsa</S100XC:digitalSignatureReference>
                <S100XC:digitalSignatureValue>
                    <S100XC:signedPublicKey id="caris">PubKey00</S100XC:signedPublicKey>
                    <S100XC:digitalSignature>12345678</S100XC:digitalSignature>
                </S100XC:digitalSignatureValue>
                <S100XC:classification gco:isoType="mco:MD_ClassificationCode">
                    <mco:MD_ClassificationCode codeList="http://standards.iso.org/iso/19115/resources/Codelists/cat/codelists.xml#MD_ClassificationCode" codeListValue="unclassified">unclassified</mco:MD_ClassificationCode>
                </S100XC:classification>
                <S100XC:purpose>new edition</S100XC:purpose>
                <S100XC:specificUsage>
                    <mri:MD_Usage>
                    <mri:specificUsage>
                        <gco:CharacterString>Overview</gco:CharacterString>
                    </mri:specificUsage>
                    </mri:MD_Usage>
                </S100XC:specificUsage>
                <S100XC:editionNumber>1</S100XC:editionNumber>
                <S100XC:updateNumber>0</S100XC:updateNumber>
                <S100XC:updateApplicationDate>2020-07-28</S100XC:updateApplicationDate>
                <S100XC:issueDate>2020-07-28</S100XC:issueDate>
                <S100XC:productSpecification>
                    <S100XC:name>S-101</S100XC:name>
                    <S100XC:version>010000</S100XC:version>
                    <S100XC:date>2018-12-01</S100XC:date>
                    <S100XC:number>101</S100XC:number>
                </S100XC:productSpecification>
                <S100XC:producingAgency>
                    <cit:CI_Responsibility>
                    <cit:role>
                        <cit:CI_RoleCode codeList="http://standards.iso.org/iso/19115/resources/Codelists/cat/codelists.xml#CI_RoleCode" codeListValue="owner">owner</cit:CI_RoleCode>
                    </cit:role>
                    <cit:party>
                        <cit:CI_Organisation>
                        <cit:name>
                            <gco:CharacterString>IHO</gco:CharacterString>
                        </cit:name>
                        </cit:CI_Organisation>
                    </cit:party>
                    </cit:CI_Responsibility>
                </S100XC:producingAgency>
                <S100XC:maximumDisplayScale>1500000</S100XC:maximumDisplayScale>
                <S100XC:horizontalDatumReference>EPSG</S100XC:horizontalDatumReference>
                <S100XC:horizontalDatumValue>4326</S100XC:horizontalDatumValue>
                <S100XC:verticalDatum>meanLowWaterSprings</S100XC:verticalDatum>
                <S100XC:soundingDatum>meanLowWaterSprings</S100XC:soundingDatum>
                <S100XC:dataType>ISO/IEC 8211</S100XC:dataType>
                <S100XC:dataTypeVersion>1.0.0</S100XC:dataTypeVersion>
                <S100XC:dataCoverage>
                    <S100XC:ID>1</S100XC:ID>
                    <S100XC:boundingBox gco:isoType="gex:EX_GeographicBoundingBox">
                    <gex:westBoundLongitude>
                        <gco:Decimal>5</gco:Decimal>
                    </gex:westBoundLongitude>
                    <gex:eastBoundLongitude>
                        <gco:Decimal>40</gco:Decimal>
                    </gex:eastBoundLongitude>
                    <gex:southBoundLatitude>
                        <gco:Decimal>76</gco:Decimal>
                    </gex:southBoundLatitude>
                    <gex:northBoundLatitude>
                        <gco:Decimal>85</gco:Decimal>
                    </gex:northBoundLatitude>
                    </S100XC:boundingBox>
                    <S100XC:boundingPolygon gco:isoType="gex:EX_BoundingPolygon">
                    <gex:polygon>
                        <gml:LineString gml:id="DC1_101AA00AA1NPOL3" srsName="urn:ogc:def:crs:EPSG::4326">
                        <gml:posList> 5.0000000 84.7566760 5.0000000 84.8649784 5.0000000 84.8649784 5.0000000 85.0000000 7.3484559 85.0000000 7.3484559 85.0000000 8.9880969 85.0000000 8.9880969 85.0000000 13.9837828 85.0000000 13.9837828 85.0000000 14.9054455 85.0000000 14.9054455 85.0000000 19.3402594 85.0000000 19.3402594 85.0000000 22.5000000 85.0000000 22.5000000 85.0000000 27.5000000 85.0000000 27.5000000 85.0000000 40.0000000 85.0000000 40.0000000 84.3488248 40.0000000 84.3488248 40.0000000 82.9752603 40.0000000 82.9752603 40.0000000 82.4249223 40.0000000 82.4249223 40.0000000 82.2098799 40.0000000 82.2098799 40.0000000 80.6946136 40.0000000 80.6946136 40.0000000 80.2669705 40.0000000 80.2669705 40.0000000 79.0649133 40.0000000 79.0649133 40.0000000 78.7213208 40.0000000 78.7213208 40.0000000 77.2716163 40.0000000 77.2716163 40.0000000 76.6815898 40.0000000 76.6815898 40.0000000 76.0000000 27.0302371 76.0000000 27.0302371 76.0000000 22.0871830 76.0000000 22.0871830 76.0000000 21.5537455 76.0000000 21.5537455 76.0000000 18.3999960 76.0000000 18.3999960 76.0000000 15.7344751 76.0000000 15.7344751 76.0000000 13.9048495 76.0000000 13.9048495 76.0000000 11.9500009 76.0000000 11.9500009 76.0000000 7.5059340 76.0000000 7.5059340 76.0000000 7.2577263 76.0000000 7.2577263 76.0000000 6.8400693 76.0000000 6.8400693 76.0000000 6.4873666 76.0000000 6.4873666 76.0000000 5.4165291 76.0000000 5.4165291 76.0000000 5.0000000 76.0000000 5.0000000 77.1025106 5.0000000 77.1025106 5.0000000 78.8023089 5.0000000 78.8023089 5.0000000 79.1915145 5.0000000 79.1915145 5.0000000 79.3204194 5.0000000 79.3204194 5.0000000 79.3600832 5.0000000 79.3600832 5.0000000 79.5311215 5.0000000 79.5311215 5.0000000 79.7835393 5.0000000 79.7835393 5.0000000 79.8956829 5.0000000 79.8956829 5.0000000 80.2580592 5.0000000 80.2580592 5.0000000 80.4491770 5.0000000 80.4491770 5.0000000 80.5422964 5.0000000 80.5422964 5.0000000 82.0327324 5.0000000 82.0327324 5.0000000 82.2727813 5.0000000 82.2727813 5.0000000 82.4530831 5.0000000 82.4530831 5.0000000 83.5606851 5.0000000 83.5606851 5.0000000 83.7246049 5.0000000 83.7246049 5.0000000 84.2107955 5.0000000 84.2107955 5.0000000 84.6203741 5.0000000 84.6203741 5.0000000 84.6505357 5.0000000 84.6505357 5.0000000 84.7566760</gml:posList>
                        </gml:LineString>
                    </gex:polygon>
                    </S100XC:boundingPolygon>
                    <S100XC:maximumDisplayScale>1500000</S100XC:maximumDisplayScale>
                    <S100XC:minimumDisplayScale>3500000</S100XC:minimumDisplayScale>
                </S100XC:dataCoverage>
                <S100XC:defaultLocale>
                    <lan:PT_Locale>
                    <lan:language>
                        <lan:LanguageCode codeList="http://www.iho.int/S100/4.0.0/resources/Codelists/cat/codelists.xml#S100_MD_LanguageCode" codeListValue="ENGLISH">ENGLISH</lan:LanguageCode>
                    </lan:language>
                    <lan:characterEncoding>
                        <lan:MD_CharacterSetCode codeList="http://www.iho.int/S100/4.0.0/resources/Codelists/cat/codelists.xml#S100_MD_CharacterSetCode" codeListValue="UTF-8">UTF-8</lan:MD_CharacterSetCode>
                    </lan:characterEncoding>
                    </lan:PT_Locale>
                </S100XC:defaultLocale>
                <S100XC:metadataFileIdentifier>CATALOG.XML</S100XC:metadataFileIdentifier>
                <S100XC:metadataPointOfContact>
                    <cit:CI_Responsibility>
                    <cit:role>
                        <cit:CI_RoleCode codeList="http://standards.iso.org/iso/19115/resources/Codelists/cat/codelists.xml#CI_RoleCode" codeListValue="owner">owner</cit:CI_RoleCode>
                    </cit:role>
                    <cit:party>
                        <cit:CI_Organisation>
                        <cit:name>
                            <gco:CharacterString>IHO</gco:CharacterString>
                        </cit:name>
                        </cit:CI_Organisation>
                    </cit:party>
                    </cit:CI_Responsibility>
                </S100XC:metadataPointOfContact>
                <S100XC:metadataDateStamp>2020-07-10</S100XC:metadataDateStamp>
                <S100XC:metadataLanguage>ENGLISH</S100XC:metadataLanguage>
                <S100XC:supportFileDiscoveryMetadataReference/>
                <S100XC:S100_19115DatasetMetadata/>
            </S100XC:S100_DatasetDiscoveryMetadata>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = DatasetDiscoveryMetadata::parse(node).unwrap();

        assert_eq!(target.file_name(), "101AA00AA1NPOL3.000");
        assert_eq!(target.file_path(), "101AA00AA1NPOL3");
        assert_eq!(target.description(), "AA1NPOL3V41");
        assert_eq!(target.data_type(), DataFormat::ISO8211);
        assert_eq!(target.data_type_version(), "1.0.0");
    }
}
