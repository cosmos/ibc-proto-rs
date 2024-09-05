impl serde::Serialize for Equivocation {
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
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.evidence.v1beta1.Equivocation", len)?;
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ::alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("power", ::alloc::string::ToString::to_string(&self.power).as_str())?;
        }
        if true {
            struct_ser.serialize_field("consensusAddress", &self.consensus_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Equivocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "time",
            "power",
            "consensus_address",
            "consensusAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Time,
            Power,
            ConsensusAddress,
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
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "power" => Ok(GeneratedField::Power),
                            "consensusAddress" | "consensus_address" => Ok(GeneratedField::ConsensusAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Equivocation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.evidence.v1beta1.Equivocation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Equivocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut time__ = None;
                let mut power__ = None;
                let mut consensus_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Power => {
                            if power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("power"));
                            }
                            power__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConsensusAddress => {
                            if consensus_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusAddress"));
                            }
                            consensus_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Equivocation {
                    height: height__.unwrap_or_default(),
                    time: time__,
                    power: power__.unwrap_or_default(),
                    consensus_address: consensus_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.evidence.v1beta1.Equivocation", FIELDS, GeneratedVisitor)
    }
}
