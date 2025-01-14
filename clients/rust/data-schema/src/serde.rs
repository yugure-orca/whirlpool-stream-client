pub mod u64_as_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(data: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&data.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        u64::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod u128_as_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(data: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&data.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        u128::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_u64_as_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(data: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // must be Some
        // skip_serializing_if = "Option::is_none" is must
        serializer.serialize_str(&data.unwrap().to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // must be Some
        // default = "Option::default" is must
        let s = String::deserialize(deserializer)?;
        Ok(Some(u64::from_str(&s).map_err(serde::de::Error::custom)?))
    }
}

pub mod big_decimal_as_string {
    use bigdecimal::BigDecimal;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    const DECIMAL_PRICE_PRECISION: u64 = 10;

    pub fn serialize<S>(data: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &data
                .with_prec(DECIMAL_PRICE_PRECISION)
                .to_scientific_notation(),
        )
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BigDecimal::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod vec_u8_as_base64_string {
    use base64::prelude::{Engine as _, BASE64_STANDARD};
    use serde::de;

    pub fn serialize<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let base64: String = BASE64_STANDARD.encode(data);
        serializer.serialize_str(&base64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let base64: String = de::Deserialize::deserialize(deserializer)?;
        match BASE64_STANDARD.decode(base64).ok() {
            Some(data) => Ok(data),
            None => Err(de::Error::custom("expected base64 string")),
        }
    }
}
