use zarrs_chunk_key_encoding::{ChunkKeyEncoding, ChunkKeyEncodingPlugin, ChunkKeyEncodingTraits};
use zarrs_metadata::v3::MetadataV3;
use zarrs_metadata::{Configuration, ConfigurationSerialize};
use zarrs_plugin::{PluginConfigurationInvalidError, PluginCreateError};
use zarrs_storage::StoreKey;

zarrs_plugin::impl_extension_aliases!(GenericChunkKeyEncoding, v3: "generic", ["zarrs:generic"]);

// Register the chunk key encoding.
inventory::submit! {
    ChunkKeyEncodingPlugin::new::<GenericChunkKeyEncoding>()
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenericChunkKeyEncodingConfiguration {
    format: String,
    separator: Option<String>,
}

impl ConfigurationSerialize for GenericChunkKeyEncodingConfiguration {}

#[derive(Debug, Clone)]
pub struct GenericChunkKeyEncoding {
    format: String,
    interpolator: crate::Interpolator,
}

impl GenericChunkKeyEncoding {
    pub fn try_new(
        format: impl Into<String>,
        separator: Option<impl Into<String>>,
    ) -> Result<Self, String> {
        let format = format.into();
        let interpolator = crate::Interpolator::try_new(&format, separator.map(|s| s.into()))?;
        Ok(Self {
            format,
            interpolator,
        })
    }
}

impl TryFrom<GenericChunkKeyEncodingConfiguration> for GenericChunkKeyEncoding {
    type Error = String;

    fn try_from(config: GenericChunkKeyEncodingConfiguration) -> Result<Self, Self::Error> {
        Self::try_new(config.format, config.separator)
    }
}

impl From<GenericChunkKeyEncoding> for GenericChunkKeyEncodingConfiguration {
    fn from(value: GenericChunkKeyEncoding) -> Self {
        let separator = value
            .interpolator
            .has_catchall()
            .then(|| value.interpolator.sep.clone());
        Self {
            format: value.format,
            separator,
        }
    }
}

impl From<&GenericChunkKeyEncoding> for GenericChunkKeyEncodingConfiguration {
    fn from(value: &GenericChunkKeyEncoding) -> Self {
        let separator = value
            .interpolator
            .has_catchall()
            .then(|| value.interpolator.sep.clone());
        Self {
            format: value.format.clone(),

            separator,
        }
    }
}

impl ChunkKeyEncodingTraits for GenericChunkKeyEncoding {
    fn create(metadata: &MetadataV3) -> Result<ChunkKeyEncoding, PluginCreateError>
    where
        Self: Sized,
    {
        let configuration: GenericChunkKeyEncodingConfiguration =
            metadata.to_typed_configuration()?;
        let generic = GenericChunkKeyEncoding::try_from(configuration).map_err(|e| {
            PluginCreateError::ConfigurationInvalid(PluginConfigurationInvalidError::new(e))
        })?;
        Ok(generic.into())
    }

    fn configuration(&self) -> Configuration {
        GenericChunkKeyEncodingConfiguration::from(self).into()
    }

    fn encode(&self, chunk_grid_indices: &[u64]) -> StoreKey {
        let key = self
            .interpolator
            .interpolate(chunk_grid_indices)
            .expect("Failed to interpolate chunk key");
        StoreKey::new(key).expect("Interpolated string is not a valid chunk key")
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use zarrs_plugin::ExtensionName;

    use super::*;
    use crate::tests::FORMAT;

    #[test]
    fn can_deser() {
        let json = serde_json::json!({
            "name": "generic",
            "configuration": {
                "format": FORMAT,
                "separator": ":"
            }
        });
        let meta = MetadataV3::deserialize(json).expect("failed to deserialize metadata");
        let config: GenericChunkKeyEncodingConfiguration = meta
            .to_typed_configuration()
            .expect("failed to deserialize typed configuration");
        let _encoding =
            GenericChunkKeyEncoding::try_from(config).expect("failed to create encoder");
    }

    #[test]
    fn can_ser() {
        let encoding =
            GenericChunkKeyEncoding::try_new(FORMAT, Some(":")).expect("failed to create encoder");
        let config: GenericChunkKeyEncodingConfiguration = (&encoding).into();
        let meta = MetadataV3::new_with_serializable_configuration(
            encoding
                .name(zarrs_plugin::ZarrVersion::V3)
                .unwrap()
                .to_string(),
            &config,
        )
        .expect("failed to serialize typed configuration");
        let json = serde_json::to_value(&meta).expect("failed to serialize metadata");
        let expected_json = serde_json::json!({
                "name": "generic",
                "configuration": {
                    "format": FORMAT,
                    "separator": ":"}
        });
        assert_eq!(json, expected_json);
    }
}
