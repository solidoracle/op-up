use async_trait::async_trait;
use eyre::Result;
use op_primitives::{Artifacts, Monorepo};
use std::process::Command;
use std::sync::Arc;

/// Devnet Allocs Stage
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Allocs {
    artifacts: Arc<Artifacts>,
    monorepo: Arc<Monorepo>,
}

#[async_trait]
impl crate::Stage for Allocs {
    /// Executes the allocs stage.
    async fn execute(&self) -> Result<()> {
        tracing::info!(target: "stages", "Executing allocs stage");

        let l2_genesis_file = self.monorepo.l2_genesis();
        if l2_genesis_file.exists() {
            tracing::info!(target: "stages", "l2 genesis file already found");
            return Ok(());
        }

        let allocs = Command::new("make")
            .args(["devnet-allocs"])
            .current_dir(self.monorepo.path())
            .output()?;
        if !allocs.status.success() {
            eyre::bail!(
                "failed to generate devnet allocs: {}",
                String::from_utf8_lossy(&allocs.stderr)
            );
        }

        let addresses = self.monorepo.addresses_json();
        self.artifacts.copy_from(&addresses)?;
        let l1_allocs = self.monorepo.allocs();
        self.artifacts.copy_from(&l1_allocs)?;

        Ok(())
    }
}

impl Allocs {
    /// Creates a new stage.
    pub fn new(artifacts: Arc<Artifacts>, monorepo: Arc<Monorepo>) -> Self {
        Self {
            artifacts,
            monorepo,
        }
    }
}
