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
        let mut struct_ser = serializer.serialize_struct("ibc.applications.packet_forward_middleware.v1.GenesisState", len)?;
        if true {
            struct_ser.serialize_field("inFlightPackets", &self.in_flight_packets)?;
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
            "in_flight_packets",
            "inFlightPackets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InFlightPackets,
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
                            "inFlightPackets" | "in_flight_packets" => Ok(GeneratedField::InFlightPackets),
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
                formatter.write_str("struct ibc.applications.packet_forward_middleware.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut in_flight_packets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InFlightPackets => {
                            if in_flight_packets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inFlightPackets"));
                            }
                            in_flight_packets__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(GenesisState {
                    in_flight_packets: in_flight_packets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.packet_forward_middleware.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InFlightPacket {
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
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.applications.packet_forward_middleware.v1.InFlightPacket", len)?;
        if true {
            struct_ser.serialize_field("originalSenderAddress", &self.original_sender_address)?;
        }
        if true {
            struct_ser.serialize_field("refundChannelId", &self.refund_channel_id)?;
        }
        if true {
            struct_ser.serialize_field("refundPortId", &self.refund_port_id)?;
        }
        if true {
            struct_ser.serialize_field("packetSrcChannelId", &self.packet_src_channel_id)?;
        }
        if true {
            struct_ser.serialize_field("packetSrcPortId", &self.packet_src_port_id)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("packetTimeoutTimestamp", ::alloc::string::ToString::to_string(&self.packet_timeout_timestamp).as_str())?;
        }
        if true {
            struct_ser.serialize_field("packetTimeoutHeight", &self.packet_timeout_height)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("packetData", pbjson::private::base64::encode(&self.packet_data).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("refundSequence", ::alloc::string::ToString::to_string(&self.refund_sequence).as_str())?;
        }
        if true {
            struct_ser.serialize_field("retriesRemaining", &self.retries_remaining)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timeout", ::alloc::string::ToString::to_string(&self.timeout).as_str())?;
        }
        if true {
            struct_ser.serialize_field("nonrefundable", &self.nonrefundable)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InFlightPacket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "original_sender_address",
            "originalSenderAddress",
            "refund_channel_id",
            "refundChannelId",
            "refund_port_id",
            "refundPortId",
            "packet_src_channel_id",
            "packetSrcChannelId",
            "packet_src_port_id",
            "packetSrcPortId",
            "packet_timeout_timestamp",
            "packetTimeoutTimestamp",
            "packet_timeout_height",
            "packetTimeoutHeight",
            "packet_data",
            "packetData",
            "refund_sequence",
            "refundSequence",
            "retries_remaining",
            "retriesRemaining",
            "timeout",
            "nonrefundable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginalSenderAddress,
            RefundChannelId,
            RefundPortId,
            PacketSrcChannelId,
            PacketSrcPortId,
            PacketTimeoutTimestamp,
            PacketTimeoutHeight,
            PacketData,
            RefundSequence,
            RetriesRemaining,
            Timeout,
            Nonrefundable,
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
                            "originalSenderAddress" | "original_sender_address" => Ok(GeneratedField::OriginalSenderAddress),
                            "refundChannelId" | "refund_channel_id" => Ok(GeneratedField::RefundChannelId),
                            "refundPortId" | "refund_port_id" => Ok(GeneratedField::RefundPortId),
                            "packetSrcChannelId" | "packet_src_channel_id" => Ok(GeneratedField::PacketSrcChannelId),
                            "packetSrcPortId" | "packet_src_port_id" => Ok(GeneratedField::PacketSrcPortId),
                            "packetTimeoutTimestamp" | "packet_timeout_timestamp" => Ok(GeneratedField::PacketTimeoutTimestamp),
                            "packetTimeoutHeight" | "packet_timeout_height" => Ok(GeneratedField::PacketTimeoutHeight),
                            "packetData" | "packet_data" => Ok(GeneratedField::PacketData),
                            "refundSequence" | "refund_sequence" => Ok(GeneratedField::RefundSequence),
                            "retriesRemaining" | "retries_remaining" => Ok(GeneratedField::RetriesRemaining),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "nonrefundable" => Ok(GeneratedField::Nonrefundable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InFlightPacket;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct ibc.applications.packet_forward_middleware.v1.InFlightPacket")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<InFlightPacket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut original_sender_address__ = None;
                let mut refund_channel_id__ = None;
                let mut refund_port_id__ = None;
                let mut packet_src_channel_id__ = None;
                let mut packet_src_port_id__ = None;
                let mut packet_timeout_timestamp__ = None;
                let mut packet_timeout_height__ = None;
                let mut packet_data__ = None;
                let mut refund_sequence__ = None;
                let mut retries_remaining__ = None;
                let mut timeout__ = None;
                let mut nonrefundable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OriginalSenderAddress => {
                            if original_sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalSenderAddress"));
                            }
                            original_sender_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefundChannelId => {
                            if refund_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refundChannelId"));
                            }
                            refund_channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefundPortId => {
                            if refund_port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refundPortId"));
                            }
                            refund_port_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PacketSrcChannelId => {
                            if packet_src_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetSrcChannelId"));
                            }
                            packet_src_channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PacketSrcPortId => {
                            if packet_src_port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetSrcPortId"));
                            }
                            packet_src_port_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PacketTimeoutTimestamp => {
                            if packet_timeout_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetTimeoutTimestamp"));
                            }
                            packet_timeout_timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PacketTimeoutHeight => {
                            if packet_timeout_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetTimeoutHeight"));
                            }
                            packet_timeout_height__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PacketData => {
                            if packet_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetData"));
                            }
                            packet_data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RefundSequence => {
                            if refund_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refundSequence"));
                            }
                            refund_sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RetriesRemaining => {
                            if retries_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriesRemaining"));
                            }
                            retries_remaining__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nonrefundable => {
                            if nonrefundable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonrefundable"));
                            }
                            nonrefundable__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InFlightPacket {
                    original_sender_address: original_sender_address__.unwrap_or_default(),
                    refund_channel_id: refund_channel_id__.unwrap_or_default(),
                    refund_port_id: refund_port_id__.unwrap_or_default(),
                    packet_src_channel_id: packet_src_channel_id__.unwrap_or_default(),
                    packet_src_port_id: packet_src_port_id__.unwrap_or_default(),
                    packet_timeout_timestamp: packet_timeout_timestamp__.unwrap_or_default(),
                    packet_timeout_height: packet_timeout_height__.unwrap_or_default(),
                    packet_data: packet_data__.unwrap_or_default(),
                    refund_sequence: refund_sequence__.unwrap_or_default(),
                    retries_remaining: retries_remaining__.unwrap_or_default(),
                    timeout: timeout__.unwrap_or_default(),
                    nonrefundable: nonrefundable__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.applications.packet_forward_middleware.v1.InFlightPacket", FIELDS, GeneratedVisitor)
    }
}
