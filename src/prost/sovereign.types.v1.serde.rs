impl serde::Serialize for AggregatedProof {
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
        let mut struct_ser = serializer.serialize_struct("sovereign.types.v1.AggregatedProof", len)?;
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("proof", pbjson::private::base64::encode(&self.proof).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregatedProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Proof,
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
                            "proof" => Ok(GeneratedField::Proof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggregatedProof;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct sovereign.types.v1.AggregatedProof")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<AggregatedProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AggregatedProof {
                    proof: proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sovereign.types.v1.AggregatedProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AggregatedProofData {
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
        let mut struct_ser = serializer.serialize_struct("sovereign.types.v1.AggregatedProofData", len)?;
        if let Some(v) = self.public_input.as_ref() {
            struct_ser.serialize_field("publicInput", v)?;
        }
        if let Some(v) = self.proof_data_info.as_ref() {
            struct_ser.serialize_field("proofDataInfo", v)?;
        }
        if let Some(v) = self.aggregated_proof.as_ref() {
            struct_ser.serialize_field("aggregatedProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregatedProofData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_input",
            "publicInput",
            "proof_data_info",
            "proofDataInfo",
            "aggregated_proof",
            "aggregatedProof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicInput,
            ProofDataInfo,
            AggregatedProof,
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
                            "publicInput" | "public_input" => Ok(GeneratedField::PublicInput),
                            "proofDataInfo" | "proof_data_info" => Ok(GeneratedField::ProofDataInfo),
                            "aggregatedProof" | "aggregated_proof" => Ok(GeneratedField::AggregatedProof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggregatedProofData;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct sovereign.types.v1.AggregatedProofData")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<AggregatedProofData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_input__ = None;
                let mut proof_data_info__ = None;
                let mut aggregated_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicInput => {
                            if public_input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicInput"));
                            }
                            public_input__ = map_.next_value()?;
                        }
                        GeneratedField::ProofDataInfo => {
                            if proof_data_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofDataInfo"));
                            }
                            proof_data_info__ = map_.next_value()?;
                        }
                        GeneratedField::AggregatedProof => {
                            if aggregated_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregatedProof"));
                            }
                            aggregated_proof__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AggregatedProofData {
                    public_input: public_input__,
                    proof_data_info: proof_data_info__,
                    aggregated_proof: aggregated_proof__,
                })
            }
        }
        deserializer.deserialize_struct("sovereign.types.v1.AggregatedProofData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProofDataInfo {
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
        let mut struct_ser = serializer.serialize_struct("sovereign.types.v1.ProofDataInfo", len)?;
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("initialStateHeight", ::alloc::string::ToString::to_string(&self.initial_state_height).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("finalStateHeight", ::alloc::string::ToString::to_string(&self.final_state_height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProofDataInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initial_state_height",
            "initialStateHeight",
            "final_state_height",
            "finalStateHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitialStateHeight,
            FinalStateHeight,
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
                            "initialStateHeight" | "initial_state_height" => Ok(GeneratedField::InitialStateHeight),
                            "finalStateHeight" | "final_state_height" => Ok(GeneratedField::FinalStateHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProofDataInfo;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct sovereign.types.v1.ProofDataInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<ProofDataInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initial_state_height__ = None;
                let mut final_state_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InitialStateHeight => {
                            if initial_state_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStateHeight"));
                            }
                            initial_state_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FinalStateHeight => {
                            if final_state_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalStateHeight"));
                            }
                            final_state_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProofDataInfo {
                    initial_state_height: initial_state_height__.unwrap_or_default(),
                    final_state_height: final_state_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sovereign.types.v1.ProofDataInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicInput {
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
        let mut struct_ser = serializer.serialize_struct("sovereign.types.v1.PublicInput", len)?;
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("initialDaBlockHash", pbjson::private::base64::encode(&self.initial_da_block_hash).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("finalDaBlockHash", pbjson::private::base64::encode(&self.final_da_block_hash).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("genesisStateRoot", pbjson::private::base64::encode(&self.genesis_state_root).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("initialStateRoot", pbjson::private::base64::encode(&self.initial_state_root).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("finalStateRoot", pbjson::private::base64::encode(&self.final_state_root).as_str())?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeCommitment", pbjson::private::base64::encode(&self.code_commitment).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initial_da_block_hash",
            "initialDaBlockHash",
            "final_da_block_hash",
            "finalDaBlockHash",
            "genesis_state_root",
            "genesisStateRoot",
            "initial_state_root",
            "initialStateRoot",
            "final_state_root",
            "finalStateRoot",
            "code_commitment",
            "codeCommitment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitialDaBlockHash,
            FinalDaBlockHash,
            GenesisStateRoot,
            InitialStateRoot,
            FinalStateRoot,
            CodeCommitment,
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
                            "initialDaBlockHash" | "initial_da_block_hash" => Ok(GeneratedField::InitialDaBlockHash),
                            "finalDaBlockHash" | "final_da_block_hash" => Ok(GeneratedField::FinalDaBlockHash),
                            "genesisStateRoot" | "genesis_state_root" => Ok(GeneratedField::GenesisStateRoot),
                            "initialStateRoot" | "initial_state_root" => Ok(GeneratedField::InitialStateRoot),
                            "finalStateRoot" | "final_state_root" => Ok(GeneratedField::FinalStateRoot),
                            "codeCommitment" | "code_commitment" => Ok(GeneratedField::CodeCommitment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicInput;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct sovereign.types.v1.PublicInput")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PublicInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initial_da_block_hash__ = None;
                let mut final_da_block_hash__ = None;
                let mut genesis_state_root__ = None;
                let mut initial_state_root__ = None;
                let mut final_state_root__ = None;
                let mut code_commitment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InitialDaBlockHash => {
                            if initial_da_block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialDaBlockHash"));
                            }
                            initial_da_block_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FinalDaBlockHash => {
                            if final_da_block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalDaBlockHash"));
                            }
                            final_da_block_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GenesisStateRoot => {
                            if genesis_state_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisStateRoot"));
                            }
                            genesis_state_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InitialStateRoot => {
                            if initial_state_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStateRoot"));
                            }
                            initial_state_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FinalStateRoot => {
                            if final_state_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalStateRoot"));
                            }
                            final_state_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CodeCommitment => {
                            if code_commitment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeCommitment"));
                            }
                            code_commitment__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PublicInput {
                    initial_da_block_hash: initial_da_block_hash__.unwrap_or_default(),
                    final_da_block_hash: final_da_block_hash__.unwrap_or_default(),
                    genesis_state_root: genesis_state_root__.unwrap_or_default(),
                    initial_state_root: initial_state_root__.unwrap_or_default(),
                    final_state_root: final_state_root__.unwrap_or_default(),
                    code_commitment: code_commitment__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sovereign.types.v1.PublicInput", FIELDS, GeneratedVisitor)
    }
}
