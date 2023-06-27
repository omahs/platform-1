use crate::ProtocolError;
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::data_contract::document_type::index::v0::{IndexPropertyV0, IndexV0};

impl IndexV0 {
    pub fn random(
        field_names: &[String],
        existing_indices: &[IndexV0],
        rng: &mut StdRng,
    ) -> Result<Self, ProtocolError> {
        let index_name = format!("index_{}", rng.gen::<u16>());

        let mut properties;
        let mut attempts = 0;
        let max_attempts = 1000; // You can adjust this value based on your requirements

        loop {
            let num_properties = rng.gen_range(1..=field_names.len());
            let mut selected_fields = field_names
                .choose_multiple(rng, num_properties)
                .cloned()
                .collect::<Vec<_>>();

            properties = selected_fields
                .drain(..)
                .map(|field_name| IndexPropertyV0 {
                    name: field_name,
                    ascending: rng.gen(),
                })
                .collect::<Vec<_>>();

            if !existing_indices
                .iter()
                .any(|index| index.properties == properties)
            {
                break;
            }

            attempts += 1;
            if attempts >= max_attempts {
                return Err(ProtocolError::Generic(
                    "Unable to generate a unique index after maximum attempts".to_string(),
                ));
            }
        }

        let unique = rng.gen();

        Ok(IndexV0 {
            name: index_name,
            properties,
            unique,
        })
    }
}