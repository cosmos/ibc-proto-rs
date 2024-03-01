impl serde::Serialize for DaData {
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
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.lightclients.rollkit.v1.DaData", len)?;
        if true {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sharedProof", pbjson::private::base64::encode(&self.shared_proof).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DaData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "shared_proof",
            "sharedProof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            SharedProof,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "sharedProof" | "shared_proof" => Ok(GeneratedField::SharedProof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DaData;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.lightclients.rollkit.v1.DaData")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DaData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut shared_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SharedProof => {
                            if shared_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedProof"));
                            }
                            shared_proof__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DaData {
                    client_id: client_id__.unwrap_or_default(),
                    shared_proof: shared_proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.lightclients.rollkit.v1.DaData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Header {
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
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.lightclients.rollkit.v1.Header", len)?;
        if let Some(v) = self.tendermint_header.as_ref() {
            struct_ser.serialize_field("tendermintHeader", v)?;
        }
        if let Some(v) = self.da_data.as_ref() {
            struct_ser.serialize_field("daData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tendermint_header",
            "tendermintHeader",
            "da_data",
            "daData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TendermintHeader,
            DaData,
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
                            "tendermintHeader" | "tendermint_header" => Ok(GeneratedField::TendermintHeader),
                            "daData" | "da_data" => Ok(GeneratedField::DaData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.lightclients.rollkit.v1.Header")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tendermint_header__ = None;
                let mut da_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TendermintHeader => {
                            if tendermint_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tendermintHeader"));
                            }
                            tendermint_header__ = map_.next_value()?;
                        }
                        GeneratedField::DaData => {
                            if da_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daData"));
                            }
                            da_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Header {
                    tendermint_header: tendermint_header__,
                    da_data: da_data__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.lightclients.rollkit.v1.Header", FIELDS, GeneratedVisitor)
    }
}
