/// Sets whether the translated text should lean towards formal or informal language.
///         This feature currently only works for target languages
///         `DE` (German),
///         `FR` (French),
///         `IT` (Italian),
///         `ES` (Spanish),
///         `NL` (Dutch),
///         `PL` (Polish),
///         `PT-PT`,
///         `PT-BR` (Portuguese)
///         and `RU` (Russian).
///       Setting this parameter with a target language that does not support formality will fail,
///         unless one of the `prefer_...` options are used.
///         Possible options are:
///           * `default` (default)
///           * `more` - for a more formal language
///           * `less` - for a more informal language
///           * `prefer_more` - for a more formal language if available, otherwise fallback to default formality
///           * `prefer_less` - for a more informal language if available, otherwise fallback to default formality
///

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Formality {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "more")]
    More,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "prefer_more")]
    PreferMore,
    #[serde(rename = "prefer_less")]
    PreferLess,
}

impl ToString for Formality {
    fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("default"),
            Self::More => String::from("more"),
            Self::Less => String::from("less"),
            Self::PreferMore => String::from("prefer_more"),
            Self::PreferLess => String::from("prefer_less"),
        }
    }
}
