impl serde::Serialize for MerklePath {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.commitment.v1.MerklePath", len)?;
        if true {
            struct_ser.serialize_field("keyPath", &self.key_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerklePath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_path",
            "keyPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyPath,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyPath" | "key_path" => Ok(GeneratedField::KeyPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerklePath;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.core.commitment.v1.MerklePath")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MerklePath, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyPath => {
                            if key_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyPath"));
                            }
                            key_path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MerklePath {
                    key_path: key_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.commitment.v1.MerklePath", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerklePrefix {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.commitment.v1.MerklePrefix", len)?;
        if true {
            struct_ser.serialize_field("keyPrefix", pbjson::private::base64::encode(&self.key_prefix).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerklePrefix {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_prefix",
            "keyPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyPrefix,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyPrefix" | "key_prefix" => Ok(GeneratedField::KeyPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerklePrefix;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.core.commitment.v1.MerklePrefix")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MerklePrefix, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyPrefix => {
                            if key_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyPrefix"));
                            }
                            key_prefix__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MerklePrefix {
                    key_prefix: key_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.commitment.v1.MerklePrefix", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.commitment.v1.MerkleProof", len)?;
        if true {
            struct_ser.serialize_field("proofs", &self.proofs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proofs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Proofs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "proofs" => Ok(GeneratedField::Proofs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleProof;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.core.commitment.v1.MerkleProof")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MerkleProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proofs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Proofs => {
                            if proofs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofs"));
                            }
                            proofs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MerkleProof {
                    proofs: proofs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.commitment.v1.MerkleProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleRoot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.commitment.v1.MerkleRoot", len)?;
        if true {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleRoot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleRoot;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.core.commitment.v1.MerkleRoot")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MerkleRoot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MerkleRoot {
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.commitment.v1.MerkleRoot", FIELDS, GeneratedVisitor)
    }
}
