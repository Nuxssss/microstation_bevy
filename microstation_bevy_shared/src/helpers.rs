use serde::{Deserialize, Deserializer};

pub fn force_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper {
        Str(String),
        Num(i64),
        Float(f64),
        Bool(bool),
    }
    let h = Helper::deserialize(deserializer)?;
    Ok(match h {
        Helper::Str(s) => s,
        Helper::Num(n) => n.to_string(),
        Helper::Float(f) => f.to_string(),
        Helper::Bool(b) => b.to_string(),
    })
}

pub fn default_true() -> bool {true}