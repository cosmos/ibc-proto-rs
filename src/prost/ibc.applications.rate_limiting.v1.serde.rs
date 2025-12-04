impl serde::Serialize for Flow {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.Flow", len)?;
        if true {
            struct_ser.serialize_field("inflow", &self.inflow)?;
        }
        if true {
            struct_ser.serialize_field("outflow", &self.outflow)?;
        }
        if true {
            struct_ser.serialize_field("channelValue", &self.channel_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Flow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inflow",
            "outflow",
            "channel_value",
            "channelValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Inflow,
            Outflow,
            ChannelValue,
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
                            "inflow" => Ok(GeneratedField::Inflow),
                            "outflow" => Ok(GeneratedField::Outflow),
                            "channelValue" | "channel_value" => Ok(GeneratedField::ChannelValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Flow;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.Flow")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Flow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inflow__ = None;
                let mut outflow__ = None;
                let mut channel_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Inflow => {
                            if inflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflow"));
                            }
                            inflow__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Outflow => {
                            if outflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outflow"));
                            }
                            outflow__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelValue => {
                            if channel_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelValue"));
                            }
                            channel_value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Flow {
                    inflow: inflow__.unwrap_or_default(),
                    outflow: outflow__.unwrap_or_default(),
                    channel_value: channel_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.Flow", FIELDS, GeneratedVisitor)
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
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.GenesisState", len)?;
        if true {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        if true {
            struct_ser.serialize_field("whitelistedAddressPairs", &self.whitelisted_address_pairs)?;
        }
        if true {
            struct_ser.serialize_field("blacklistedDenoms", &self.blacklisted_denoms)?;
        }
        if true {
            struct_ser.serialize_field("pendingSendPacketSequenceNumbers", &self.pending_send_packet_sequence_numbers)?;
        }
        if let Some(v) = self.hour_epoch.as_ref() {
            struct_ser.serialize_field("hourEpoch", v)?;
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
            "rate_limits",
            "rateLimits",
            "whitelisted_address_pairs",
            "whitelistedAddressPairs",
            "blacklisted_denoms",
            "blacklistedDenoms",
            "pending_send_packet_sequence_numbers",
            "pendingSendPacketSequenceNumbers",
            "hour_epoch",
            "hourEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimits,
            WhitelistedAddressPairs,
            BlacklistedDenoms,
            PendingSendPacketSequenceNumbers,
            HourEpoch,
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
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "whitelistedAddressPairs" | "whitelisted_address_pairs" => Ok(GeneratedField::WhitelistedAddressPairs),
                            "blacklistedDenoms" | "blacklisted_denoms" => Ok(GeneratedField::BlacklistedDenoms),
                            "pendingSendPacketSequenceNumbers" | "pending_send_packet_sequence_numbers" => Ok(GeneratedField::PendingSendPacketSequenceNumbers),
                            "hourEpoch" | "hour_epoch" => Ok(GeneratedField::HourEpoch),
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
                formatter.write_str("struct ibc.applications.rate_limiting.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limits__ = None;
                let mut whitelisted_address_pairs__ = None;
                let mut blacklisted_denoms__ = None;
                let mut pending_send_packet_sequence_numbers__ = None;
                let mut hour_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WhitelistedAddressPairs => {
                            if whitelisted_address_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("whitelistedAddressPairs"));
                            }
                            whitelisted_address_pairs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlacklistedDenoms => {
                            if blacklisted_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blacklistedDenoms"));
                            }
                            blacklisted_denoms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PendingSendPacketSequenceNumbers => {
                            if pending_send_packet_sequence_numbers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingSendPacketSequenceNumbers"));
                            }
                            pending_send_packet_sequence_numbers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HourEpoch => {
                            if hour_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hourEpoch"));
                            }
                            hour_epoch__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenesisState {
                    rate_limits: rate_limits__.unwrap_or_default(),
                    whitelisted_address_pairs: whitelisted_address_pairs__.unwrap_or_default(),
                    blacklisted_denoms: blacklisted_denoms__.unwrap_or_default(),
                    pending_send_packet_sequence_numbers: pending_send_packet_sequence_numbers__.unwrap_or_default(),
                    hour_epoch: hour_epoch__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HourEpoch {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.HourEpoch", len)?;
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epochNumber", ::alloc::string::ToString::to_string(&self.epoch_number).as_str())?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if let Some(v) = self.epoch_start_time.as_ref() {
            struct_ser.serialize_field("epochStartTime", v)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epochStartHeight", ::alloc::string::ToString::to_string(&self.epoch_start_height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HourEpoch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch_number",
            "epochNumber",
            "duration",
            "epoch_start_time",
            "epochStartTime",
            "epoch_start_height",
            "epochStartHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
            Duration,
            EpochStartTime,
            EpochStartHeight,
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
                            "epochNumber" | "epoch_number" => Ok(GeneratedField::EpochNumber),
                            "duration" => Ok(GeneratedField::Duration),
                            "epochStartTime" | "epoch_start_time" => Ok(GeneratedField::EpochStartTime),
                            "epochStartHeight" | "epoch_start_height" => Ok(GeneratedField::EpochStartHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HourEpoch;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.HourEpoch")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<HourEpoch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                let mut duration__ = None;
                let mut epoch_start_time__ = None;
                let mut epoch_start_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EpochNumber => {
                            if epoch_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochNumber"));
                            }
                            epoch_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                        GeneratedField::EpochStartTime => {
                            if epoch_start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochStartTime"));
                            }
                            epoch_start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EpochStartHeight => {
                            if epoch_start_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochStartHeight"));
                            }
                            epoch_start_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HourEpoch {
                    epoch_number: epoch_number__.unwrap_or_default(),
                    duration: duration__,
                    epoch_start_time: epoch_start_time__,
                    epoch_start_height: epoch_start_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.HourEpoch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddRateLimit {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgAddRateLimit", len)?;
        if true {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        if true {
            struct_ser.serialize_field("maxPercentSend", &self.max_percent_send)?;
        }
        if true {
            struct_ser.serialize_field("maxPercentRecv", &self.max_percent_recv)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("durationHours", ::alloc::string::ToString::to_string(&self.duration_hours).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
            "max_percent_send",
            "maxPercentSend",
            "max_percent_recv",
            "maxPercentRecv",
            "duration_hours",
            "durationHours",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Denom,
            ChannelOrClientId,
            MaxPercentSend,
            MaxPercentRecv,
            DurationHours,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "denom" => Ok(GeneratedField::Denom),
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            "maxPercentSend" | "max_percent_send" => Ok(GeneratedField::MaxPercentSend),
                            "maxPercentRecv" | "max_percent_recv" => Ok(GeneratedField::MaxPercentRecv),
                            "durationHours" | "duration_hours" => Ok(GeneratedField::DurationHours),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddRateLimit;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgAddRateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgAddRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                let mut max_percent_send__ = None;
                let mut max_percent_recv__ = None;
                let mut duration_hours__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPercentSend => {
                            if max_percent_send__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentSend"));
                            }
                            max_percent_send__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPercentRecv => {
                            if max_percent_recv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentRecv"));
                            }
                            max_percent_recv__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationHours => {
                            if duration_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationHours"));
                            }
                            duration_hours__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgAddRateLimit {
                    signer: signer__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                    max_percent_send: max_percent_send__.unwrap_or_default(),
                    max_percent_recv: max_percent_recv__.unwrap_or_default(),
                    duration_hours: duration_hours__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgAddRateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddRateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgAddRateLimitResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRateLimitResponse {
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
            type Value = MsgAddRateLimitResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgAddRateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgAddRateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddRateLimitResponse {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgAddRateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveRateLimit {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgRemoveRateLimit", len)?;
        if true {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Denom,
            ChannelOrClientId,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "denom" => Ok(GeneratedField::Denom),
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveRateLimit;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgRemoveRateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRemoveRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveRateLimit {
                    signer: signer__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgRemoveRateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveRateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgRemoveRateLimitResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRateLimitResponse {
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
            type Value = MsgRemoveRateLimitResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgRemoveRateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRemoveRateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveRateLimitResponse {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgRemoveRateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgResetRateLimit {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgResetRateLimit", len)?;
        if true {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgResetRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Denom,
            ChannelOrClientId,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "denom" => Ok(GeneratedField::Denom),
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgResetRateLimit;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgResetRateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgResetRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgResetRateLimit {
                    signer: signer__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgResetRateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgResetRateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgResetRateLimitResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgResetRateLimitResponse {
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
            type Value = MsgResetRateLimitResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgResetRateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgResetRateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgResetRateLimitResponse {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgResetRateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateRateLimit {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgUpdateRateLimit", len)?;
        if true {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        if true {
            struct_ser.serialize_field("maxPercentSend", &self.max_percent_send)?;
        }
        if true {
            struct_ser.serialize_field("maxPercentRecv", &self.max_percent_recv)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("durationHours", ::alloc::string::ToString::to_string(&self.duration_hours).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
            "max_percent_send",
            "maxPercentSend",
            "max_percent_recv",
            "maxPercentRecv",
            "duration_hours",
            "durationHours",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Denom,
            ChannelOrClientId,
            MaxPercentSend,
            MaxPercentRecv,
            DurationHours,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "denom" => Ok(GeneratedField::Denom),
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            "maxPercentSend" | "max_percent_send" => Ok(GeneratedField::MaxPercentSend),
                            "maxPercentRecv" | "max_percent_recv" => Ok(GeneratedField::MaxPercentRecv),
                            "durationHours" | "duration_hours" => Ok(GeneratedField::DurationHours),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateRateLimit;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgUpdateRateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                let mut max_percent_send__ = None;
                let mut max_percent_recv__ = None;
                let mut duration_hours__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPercentSend => {
                            if max_percent_send__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentSend"));
                            }
                            max_percent_send__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPercentRecv => {
                            if max_percent_recv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentRecv"));
                            }
                            max_percent_recv__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationHours => {
                            if duration_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationHours"));
                            }
                            duration_hours__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgUpdateRateLimit {
                    signer: signer__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                    max_percent_send: max_percent_send__.unwrap_or_default(),
                    max_percent_recv: max_percent_recv__.unwrap_or_default(),
                    duration_hours: duration_hours__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgUpdateRateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateRateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.MsgUpdateRateLimitResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateRateLimitResponse {
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
            type Value = MsgUpdateRateLimitResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.MsgUpdateRateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateRateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateRateLimitResponse {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.MsgUpdateRateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PacketDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::PacketSend => "PACKET_SEND",
            Self::PacketRecv => "PACKET_RECV",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PacketDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PACKET_SEND",
            "PACKET_RECV",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PacketDirection;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PACKET_SEND" => Ok(PacketDirection::PacketSend),
                    "PACKET_RECV" => Ok(PacketDirection::PacketRecv),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Path {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.Path", len)?;
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Path {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            ChannelOrClientId,
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
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Path;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.Path")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Path, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Path {
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.Path", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllBlacklistedDenomsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllBlacklistedDenomsRequest {
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
            type Value = QueryAllBlacklistedDenomsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllBlacklistedDenomsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllBlacklistedDenomsRequest {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllBlacklistedDenomsResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsResponse", len)?;
        if true {
            struct_ser.serialize_field("denoms", &self.denoms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllBlacklistedDenomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denoms",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denoms,
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
                            "denoms" => Ok(GeneratedField::Denoms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllBlacklistedDenomsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllBlacklistedDenomsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denoms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denoms => {
                            if denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denoms"));
                            }
                            denoms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAllBlacklistedDenomsResponse {
                    denoms: denoms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllRateLimitsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllRateLimitsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllRateLimitsRequest {
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
            type Value = QueryAllRateLimitsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllRateLimitsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllRateLimitsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllRateLimitsRequest {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllRateLimitsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllRateLimitsResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllRateLimitsResponse", len)?;
        if true {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllRateLimitsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate_limits",
            "rateLimits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimits,
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
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllRateLimitsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllRateLimitsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllRateLimitsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAllRateLimitsResponse {
                    rate_limits: rate_limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllRateLimitsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllWhitelistedAddressesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllWhitelistedAddressesRequest {
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
            type Value = QueryAllWhitelistedAddressesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllWhitelistedAddressesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllWhitelistedAddressesRequest {
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllWhitelistedAddressesResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesResponse", len)?;
        if true {
            struct_ser.serialize_field("addressPairs", &self.address_pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllWhitelistedAddressesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address_pairs",
            "addressPairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressPairs,
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
                            "addressPairs" | "address_pairs" => Ok(GeneratedField::AddressPairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllWhitelistedAddressesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllWhitelistedAddressesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address_pairs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AddressPairs => {
                            if address_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressPairs"));
                            }
                            address_pairs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAllWhitelistedAddressesResponse {
                    address_pairs: address_pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitRequest", len)?;
        if true {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "channel_or_client_id",
            "channelOrClientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            ChannelOrClientId,
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
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut channel_or_client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRateLimitRequest {
                    denom: denom__.unwrap_or_default(),
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitResponse", len)?;
        if let Some(v) = self.rate_limit.as_ref() {
            struct_ser.serialize_field("rateLimit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate_limit",
            "rateLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimit,
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
                            "rateLimit" | "rate_limit" => Ok(GeneratedField::RateLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimit => {
                            if rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimit"));
                            }
                            rate_limit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryRateLimitResponse {
                    rate_limit: rate_limit__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitsByChainIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDRequest", len)?;
        if true {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitsByChainIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitsByChainIdRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitsByChainIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRateLimitsByChainIdRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitsByChainIdResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDResponse", len)?;
        if true {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitsByChainIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate_limits",
            "rateLimits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimits,
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
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitsByChainIdResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitsByChainIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRateLimitsByChainIdResponse {
                    rate_limits: rate_limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitsByChannelOrClientIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDRequest", len)?;
        if true {
            struct_ser.serialize_field("channelOrClientId", &self.channel_or_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitsByChannelOrClientIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_or_client_id",
            "channelOrClientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelOrClientId,
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
                            "channelOrClientId" | "channel_or_client_id" => Ok(GeneratedField::ChannelOrClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitsByChannelOrClientIdRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitsByChannelOrClientIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_or_client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelOrClientId => {
                            if channel_or_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelOrClientId"));
                            }
                            channel_or_client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRateLimitsByChannelOrClientIdRequest {
                    channel_or_client_id: channel_or_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRateLimitsByChannelOrClientIdResponse {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDResponse", len)?;
        if true {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRateLimitsByChannelOrClientIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate_limits",
            "rateLimits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimits,
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
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRateLimitsByChannelOrClientIdResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryRateLimitsByChannelOrClientIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRateLimitsByChannelOrClientIdResponse {
                    rate_limits: rate_limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quota {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.Quota", len)?;
        if true {
            struct_ser.serialize_field("maxPercentSend", &self.max_percent_send)?;
        }
        if true {
            struct_ser.serialize_field("maxPercentRecv", &self.max_percent_recv)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("durationHours", ::alloc::string::ToString::to_string(&self.duration_hours).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quota {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_percent_send",
            "maxPercentSend",
            "max_percent_recv",
            "maxPercentRecv",
            "duration_hours",
            "durationHours",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxPercentSend,
            MaxPercentRecv,
            DurationHours,
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
                            "maxPercentSend" | "max_percent_send" => Ok(GeneratedField::MaxPercentSend),
                            "maxPercentRecv" | "max_percent_recv" => Ok(GeneratedField::MaxPercentRecv),
                            "durationHours" | "duration_hours" => Ok(GeneratedField::DurationHours),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quota;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.Quota")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Quota, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_percent_send__ = None;
                let mut max_percent_recv__ = None;
                let mut duration_hours__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxPercentSend => {
                            if max_percent_send__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentSend"));
                            }
                            max_percent_send__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPercentRecv => {
                            if max_percent_recv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPercentRecv"));
                            }
                            max_percent_recv__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationHours => {
                            if duration_hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationHours"));
                            }
                            duration_hours__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Quota {
                    max_percent_send: max_percent_send__.unwrap_or_default(),
                    max_percent_recv: max_percent_recv__.unwrap_or_default(),
                    duration_hours: duration_hours__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.Quota", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimit {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.RateLimit", len)?;
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        if let Some(v) = self.flow.as_ref() {
            struct_ser.serialize_field("flow", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "quota",
            "flow",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Quota,
            Flow,
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
                            "quota" => Ok(GeneratedField::Quota),
                            "flow" => Ok(GeneratedField::Flow),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.RateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut quota__ = None;
                let mut flow__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                        GeneratedField::Flow => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flow"));
                            }
                            flow__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RateLimit {
                    path: path__,
                    quota: quota__,
                    flow: flow__,
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WhitelistedAddressPair {
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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.rate_limiting.v1.WhitelistedAddressPair", len)?;
        if true {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if true {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WhitelistedAddressPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "receiver",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Receiver,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WhitelistedAddressPair;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.rate_limiting.v1.WhitelistedAddressPair")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<WhitelistedAddressPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut receiver__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                    }
                }
                Ok(WhitelistedAddressPair {
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.rate_limiting.v1.WhitelistedAddressPair", FIELDS, GeneratedVisitor)
    }
}
