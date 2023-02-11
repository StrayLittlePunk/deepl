/*
 * DeepL API Documentation
 *
 * The DeepL API provides programmatic access to DeepL’s machine translation technology.
 *
 * The version of the OpenAPI document: 2.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SourceLanguage : Language of the text to be translated. Options currently available:  * `BG` - Bulgarian  * `CS` - Czech  * `DA` - Danish  * `DE` - German  * `EL` - Greek  * `EN` - English  * `ES` - Spanish  * `ET` - Estonian  * `FI` - Finnish  * `FR` - French  * `HU` - Hungarian  * `ID` - Indonesian  * `IT` - Italian  * `JA` - Japanese  * `KO` - Korean  * `LT` - Lithuanian  * `LV` - Latvian  * `NB` - Norwegian (Bokmål)  * `NL` - Dutch  * `PL` - Polish  * `PT` - Portuguese (all Portuguese varieties mixed)  * `RO` - Romanian  * `RU` - Russian  * `SK` - Slovak  * `SL` - Slovenian  * `SV` - Swedish  * `TR` - Turkish  * `UK` - Ukrainian  * `ZH` - Chinese  If this parameter is omitted, the API will attempt to detect the language of the text and translate it.

/// Language of the text to be translated. Options currently available:  * `BG` - Bulgarian  * `CS` - Czech  * `DA` - Danish  * `DE` - German  * `EL` - Greek  * `EN` - English  * `ES` - Spanish  * `ET` - Estonian  * `FI` - Finnish  * `FR` - French  * `HU` - Hungarian  * `ID` - Indonesian  * `IT` - Italian  * `JA` - Japanese  * `KO` - Korean  * `LT` - Lithuanian  * `LV` - Latvian  * `NB` - Norwegian (Bokmål)  * `NL` - Dutch  * `PL` - Polish  * `PT` - Portuguese (all Portuguese varieties mixed)  * `RO` - Romanian  * `RU` - Russian  * `SK` - Slovak  * `SL` - Slovenian  * `SV` - Swedish  * `TR` - Turkish  * `UK` - Ukrainian  * `ZH` - Chinese  If this parameter is omitted, the API will attempt to detect the language of the text and translate it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceLanguage {
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "DA")]
    Da,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "EN")]
    En,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JA")]
    Ja,
    #[serde(rename = "KO")]
    Ko,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "NB")]
    Nb,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "UK")]
    Uk,
    #[serde(rename = "ZH")]
    Zh,

}

impl ToString for SourceLanguage {
    fn to_string(&self) -> String {
        match self {
            Self::Bg => String::from("BG"),
            Self::Cs => String::from("CS"),
            Self::Da => String::from("DA"),
            Self::De => String::from("DE"),
            Self::El => String::from("EL"),
            Self::En => String::from("EN"),
            Self::Es => String::from("ES"),
            Self::Et => String::from("ET"),
            Self::Fi => String::from("FI"),
            Self::Fr => String::from("FR"),
            Self::Hu => String::from("HU"),
            Self::Id => String::from("ID"),
            Self::It => String::from("IT"),
            Self::Ja => String::from("JA"),
            Self::Ko => String::from("KO"),
            Self::Lt => String::from("LT"),
            Self::Lv => String::from("LV"),
            Self::Nb => String::from("NB"),
            Self::Nl => String::from("NL"),
            Self::Pl => String::from("PL"),
            Self::Pt => String::from("PT"),
            Self::Ro => String::from("RO"),
            Self::Ru => String::from("RU"),
            Self::Sk => String::from("SK"),
            Self::Sl => String::from("SL"),
            Self::Sv => String::from("SV"),
            Self::Tr => String::from("TR"),
            Self::Uk => String::from("UK"),
            Self::Zh => String::from("ZH"),
        }
    }
}

impl Default for SourceLanguage {
    fn default() -> SourceLanguage {
        Self::Bg
    }
}




