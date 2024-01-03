impl serde::Serialize for ClassTrace {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.ClassTrace", len)?;
        if true {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if true {
            struct_ser.serialize_field("baseClassId", &self.base_class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "base_class_id",
            "baseClassId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            BaseClassId,
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
                            "path" => Ok(GeneratedField::Path),
                            "baseClassId" | "base_class_id" => Ok(GeneratedField::BaseClassId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassTrace;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.ClassTrace")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<ClassTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut base_class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseClassId => {
                            if base_class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseClassId"));
                            }
                            base_class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClassTrace {
                    path: path__.unwrap_or_default(),
                    base_class_id: base_class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.ClassTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisState {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.GenesisState", len)?;
        if true {
            struct_ser.serialize_field("portId", &self.port_id)?;
        }
        if true {
            struct_ser.serialize_field("traces", &self.traces)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "port_id",
            "portId",
            "traces",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortId,
            Traces,
            Params,
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
                            "portId" | "port_id" => Ok(GeneratedField::PortId),
                            "traces" => Ok(GeneratedField::Traces),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port_id__ = None;
                let mut traces__ = None;
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PortId => {
                            if port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portId"));
                            }
                            port_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Traces => {
                            if traces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traces"));
                            }
                            traces__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(GenesisState {
                    port_id: port_id__.unwrap_or_default(),
                    traces: traces__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgTransfer {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.MsgTransfer", len)?;
        if true {
            struct_ser.serialize_field("sourcePort", &self.source_port)?;
        }
        if true {
            struct_ser.serialize_field("sourceChannel", &self.source_channel)?;
        }
        if true {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if true {
            struct_ser.serialize_field("tokenIds", &self.token_ids)?;
        }
        if true {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if true {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if let Some(v) = self.timeout_height.as_ref() {
            struct_ser.serialize_field("timeoutHeight", v)?;
        }
        if true {
            struct_ser.serialize_field("timeoutTimestamp", ::alloc::string::ToString::to_string(&self.timeout_timestamp).as_str())?;
        }
        if true {
            struct_ser.serialize_field("memo", &self.memo)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_port",
            "sourcePort",
            "source_channel",
            "sourceChannel",
            "class_id",
            "classId",
            "token_ids",
            "tokenIds",
            "sender",
            "receiver",
            "timeout_height",
            "timeoutHeight",
            "timeout_timestamp",
            "timeoutTimestamp",
            "memo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourcePort,
            SourceChannel,
            ClassId,
            TokenIds,
            Sender,
            Receiver,
            TimeoutHeight,
            TimeoutTimestamp,
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
                            "sourcePort" | "source_port" => Ok(GeneratedField::SourcePort),
                            "sourceChannel" | "source_channel" => Ok(GeneratedField::SourceChannel),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "tokenIds" | "token_ids" => Ok(GeneratedField::TokenIds),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            "timeoutHeight" | "timeout_height" => Ok(GeneratedField::TimeoutHeight),
                            "timeoutTimestamp" | "timeout_timestamp" => Ok(GeneratedField::TimeoutTimestamp),
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
            type Value = MsgTransfer;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.MsgTransfer")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MsgTransfer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_port__ = None;
                let mut source_channel__ = None;
                let mut class_id__ = None;
                let mut token_ids__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                let mut timeout_height__ = None;
                let mut timeout_timestamp__ = None;
                let mut memo__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourcePort => {
                            if source_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePort"));
                            }
                            source_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceChannel => {
                            if source_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceChannel"));
                            }
                            source_channel__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenIds => {
                            if token_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenIds"));
                            }
                            token_ids__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeoutHeight => {
                            if timeout_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutHeight"));
                            }
                            timeout_height__ = map.next_value()?;
                        }
                        GeneratedField::TimeoutTimestamp => {
                            if timeout_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutTimestamp"));
                            }
                            timeout_timestamp__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Memo => {
                            if memo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memo"));
                            }
                            memo__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgTransfer {
                    source_port: source_port__.unwrap_or_default(),
                    source_channel: source_channel__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    token_ids: token_ids__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                    timeout_height: timeout_height__,
                    timeout_timestamp: timeout_timestamp__.unwrap_or_default(),
                    memo: memo__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.MsgTransfer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgTransferResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.MsgTransferResponse", len)?;
        if true {
            struct_ser.serialize_field("sequence", ::alloc::string::ToString::to_string(&self.sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.MsgTransferResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MsgTransferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgTransferResponse {
                    sequence: sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.MsgTransferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateParams {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.MsgUpdateParams", len)?;
        if true {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MsgUpdateParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.MsgUpdateParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<MsgUpdateParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.MsgUpdateParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NonFungibleTokenPacketData {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.NonFungibleTokenPacketData", len)?;
        if true {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if true {
            struct_ser.serialize_field("classUri", &self.class_uri)?;
        }
        if true {
            struct_ser.serialize_field("classData", &self.class_data)?;
        }
        if true {
            struct_ser.serialize_field("tokenIds", &self.token_ids)?;
        }
        if true {
            struct_ser.serialize_field("tokenUris", &self.token_uris)?;
        }
        if true {
            struct_ser.serialize_field("tokenData", &self.token_data)?;
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
impl<'de> serde::Deserialize<'de> for NonFungibleTokenPacketData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
            "class_uri",
            "classUri",
            "class_data",
            "classData",
            "token_ids",
            "tokenIds",
            "token_uris",
            "tokenUris",
            "token_data",
            "tokenData",
            "sender",
            "receiver",
            "memo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
            ClassUri,
            ClassData,
            TokenIds,
            TokenUris,
            TokenData,
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
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "classUri" | "class_uri" => Ok(GeneratedField::ClassUri),
                            "classData" | "class_data" => Ok(GeneratedField::ClassData),
                            "tokenIds" | "token_ids" => Ok(GeneratedField::TokenIds),
                            "tokenUris" | "token_uris" => Ok(GeneratedField::TokenUris),
                            "tokenData" | "token_data" => Ok(GeneratedField::TokenData),
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
            type Value = NonFungibleTokenPacketData;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.NonFungibleTokenPacketData")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<NonFungibleTokenPacketData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut class_uri__ = None;
                let mut class_data__ = None;
                let mut token_ids__ = None;
                let mut token_uris__ = None;
                let mut token_data__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                let mut memo__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassUri => {
                            if class_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classUri"));
                            }
                            class_uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassData => {
                            if class_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classData"));
                            }
                            class_data__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenIds => {
                            if token_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenIds"));
                            }
                            token_ids__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenUris => {
                            if token_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenUris"));
                            }
                            token_uris__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenData => {
                            if token_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenData"));
                            }
                            token_data__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                        GeneratedField::Memo => {
                            if memo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memo"));
                            }
                            memo__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(NonFungibleTokenPacketData {
                    class_id: class_id__.unwrap_or_default(),
                    class_uri: class_uri__.unwrap_or_default(),
                    class_data: class_data__.unwrap_or_default(),
                    token_ids: token_ids__.unwrap_or_default(),
                    token_uris: token_uris__.unwrap_or_default(),
                    token_data: token_data__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                    memo: memo__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.NonFungibleTokenPacketData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Params {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.Params", len)?;
        if true {
            struct_ser.serialize_field("sendEnabled", &self.send_enabled)?;
        }
        if true {
            struct_ser.serialize_field("receiveEnabled", &self.receive_enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "send_enabled",
            "sendEnabled",
            "receive_enabled",
            "receiveEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SendEnabled,
            ReceiveEnabled,
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
                            "sendEnabled" | "send_enabled" => Ok(GeneratedField::SendEnabled),
                            "receiveEnabled" | "receive_enabled" => Ok(GeneratedField::ReceiveEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut send_enabled__ = None;
                let mut receive_enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SendEnabled => {
                            if send_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendEnabled"));
                            }
                            send_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReceiveEnabled => {
                            if receive_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiveEnabled"));
                            }
                            receive_enabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    send_enabled: send_enabled__.unwrap_or_default(),
                    receive_enabled: receive_enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassHashRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassHashRequest", len)?;
        if true {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryClassHashRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassHashRequest")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryClassHashRequest {
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassHashRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassHashResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassHashResponse", len)?;
        if true {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassHashResponse {
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
            type Value = QueryClassHashResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassHashResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassHashResponse, V::Error>
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
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryClassHashResponse {
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassHashResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassTraceRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassTraceRequest", len)?;
        if true {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassTraceRequest {
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
            type Value = QueryClassTraceRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassTraceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassTraceRequest, V::Error>
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
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryClassTraceRequest {
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassTraceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassTraceResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassTraceResponse", len)?;
        if let Some(v) = self.class_trace.as_ref() {
            struct_ser.serialize_field("classTrace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassTraceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_trace",
            "classTrace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassTrace,
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
                            "classTrace" | "class_trace" => Ok(GeneratedField::ClassTrace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassTraceResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassTraceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassTraceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassTrace => {
                            if class_trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classTrace"));
                            }
                            class_trace__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassTraceResponse {
                    class_trace: class_trace__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassTraceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassTracesRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassTracesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassTracesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassTracesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassTracesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassTracesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassTracesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassTracesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassTracesResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryClassTracesResponse", len)?;
        if true {
            struct_ser.serialize_field("classTraces", &self.class_traces)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassTracesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_traces",
            "classTraces",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassTraces,
            Pagination,
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
                            "classTraces" | "class_traces" => Ok(GeneratedField::ClassTraces),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassTracesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryClassTracesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryClassTracesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_traces__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassTraces => {
                            if class_traces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classTraces"));
                            }
                            class_traces__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassTracesResponse {
                    class_traces: class_traces__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryClassTracesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEscrowAddressRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryEscrowAddressRequest", len)?;
        if true {
            struct_ser.serialize_field("portId", &self.port_id)?;
        }
        if true {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEscrowAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "port_id",
            "portId",
            "channel_id",
            "channelId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortId,
            ChannelId,
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
                            "portId" | "port_id" => Ok(GeneratedField::PortId),
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEscrowAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryEscrowAddressRequest")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryEscrowAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port_id__ = None;
                let mut channel_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PortId => {
                            if port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portId"));
                            }
                            port_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryEscrowAddressRequest {
                    port_id: port_id__.unwrap_or_default(),
                    channel_id: channel_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryEscrowAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEscrowAddressResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryEscrowAddressResponse", len)?;
        if true {
            struct_ser.serialize_field("escrowAddress", &self.escrow_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEscrowAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "escrow_address",
            "escrowAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EscrowAddress,
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
                            "escrowAddress" | "escrow_address" => Ok(GeneratedField::EscrowAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEscrowAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryEscrowAddressResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryEscrowAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut escrow_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EscrowAddress => {
                            if escrow_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("escrowAddress"));
                            }
                            escrow_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryEscrowAddressResponse {
                    escrow_address: escrow_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryEscrowAddressResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryParamsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.nft_transfer.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.nft_transfer.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> core::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.nft_transfer.v1.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
