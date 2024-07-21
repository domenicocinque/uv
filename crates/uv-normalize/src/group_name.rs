use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::string::InternedString;
use crate::{validate_and_normalize_owned, validate_and_normalize_ref, InvalidNameError};

/// The normalized name of a dependency group.
///
/// See:
/// - <https://peps.python.org/pep-0735/>
/// - <https://packaging.python.org/en/latest/specifications/name-normalization/>
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct GroupName(InternedString);

impl GroupName {
    /// Create a validated, normalized extra name.
    pub fn new(name: String) -> Result<Self, InvalidNameError> {
        validate_and_normalize_owned(name)
            .map(InternedString::from)
            .map(Self)
    }
}

impl FromStr for GroupName {
    type Err = InvalidNameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        validate_and_normalize_ref(name)
            .map(InternedString::from)
            .map(Self)
    }
}

impl<'de> Deserialize<'de> for GroupName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Display for GroupName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for GroupName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for GroupName {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }
}
