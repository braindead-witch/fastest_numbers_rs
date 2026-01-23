use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::{math::expression::Number, syllables::counter::NumberRepresentation};

#[derive(Serialize, Deserialize)]
pub struct OptimizedResult {
    pub inner: HashMap<Number, NumberRepresentation>,
}

pub enum ExportType {
    Json,
}

impl OptimizedResult {
    pub fn export<P: AsRef<Path>>(&self, flavor: ExportType, filepath: P) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        match flavor {
            ExportType::Json => {
                let mut sorted_inner: Vec<_> = self.inner
                    .iter()
                    .collect();
                sorted_inner.sort_by_key(|k| k.0);
                serde_json::to_writer_pretty(file, &sorted_inner)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(())
            },
        }
    }
}
