impl serde::Serialize for FungibleTokenPacketData {
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
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.applications.transfer.v2.FungibleTokenPacketData", len)?;
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if true {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if true {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if true {
            struct_ser.serialize_field("memo", &self.memo)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FungibleTokenPacketData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "amount",
            "sender",
            "receiver",
            "memo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            Amount,
            Sender,
            Receiver,
            Memo,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "amount" => Ok(GeneratedField::Amount),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            "memo" => Ok(GeneratedField::Memo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FungibleTokenPacketData;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.transfer.v2.FungibleTokenPacketData")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<FungibleTokenPacketData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut amount__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                let mut memo__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Memo => {
                            if memo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memo"));
                            }
                            memo__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FungibleTokenPacketData {
                    denom: denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                    memo: memo__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.transfer.v2.FungibleTokenPacketData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FungibleTokenPacketDataV2 {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.transfer.v2.FungibleTokenPacketDataV2", len)?;
        if true {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        if true {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if true {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if true {
            struct_ser.serialize_field("memo", &self.memo)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FungibleTokenPacketDataV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
            "sender",
            "receiver",
            "memo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
            Sender,
            Receiver,
            Memo,
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
                            "tokens" => Ok(GeneratedField::Tokens),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            "memo" => Ok(GeneratedField::Memo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FungibleTokenPacketDataV2;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.transfer.v2.FungibleTokenPacketDataV2")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<FungibleTokenPacketDataV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                let mut memo__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Memo => {
                            if memo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memo"));
                            }
                            memo__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FungibleTokenPacketDataV2 {
                    tokens: tokens__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                    memo: memo__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.transfer.v2.FungibleTokenPacketDataV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Token {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.transfer.v2.Token", len)?;
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if true {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Token {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "amount",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            Amount,
            Trace,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "amount" => Ok(GeneratedField::Amount),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Token;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.transfer.v2.Token")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Token, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut amount__ = None;
                let mut trace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Token {
                    denom: denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.transfer.v2.Token", FIELDS, GeneratedVisitor)
    }
}
