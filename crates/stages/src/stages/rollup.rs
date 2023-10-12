use eyre::Result;
use std::process::Command;

/// Rollup Stage
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rollup {
    /// The rollup client choice.
    pub rollup_client: String,
}

impl crate::Stage for Rollup {
    /// Executes the [Rollup] stage.
    fn execute(&self) -> Result<()> {
        tracing::info!(target: "stages", "Executing rollup stage");

        // todo: this should be replaced with running the docker container inline through
        // the op-composer crate anyways so we won't need the docker directory at all.
        let proj_root = project_root::get_project_root()?;
        let docker_dir = proj_root.as_path().join("docker");

        tracing::info!(target: "stages", "Starting rollup client {}", &self.rollup_client);
        let start_rollup = Command::new("docker-compose")
            .args(["up", "-d", "--no-deps", "--build", "rollup-client"])
            .env("PWD", &docker_dir)
            .env("ROLLUP_CLIENT_CHOICE", &self.rollup_client)
            .current_dir(docker_dir)
            .output()?;

        if !start_rollup.status.success() {
            eyre::bail!(
                "failed to start rollup client: {}",
                String::from_utf8_lossy(&start_rollup.stderr)
            );
        }

        crate::net::wait_up(op_config::ROLLUP_PORT, 30, 1)?;

        Ok(())
    }
}

impl Rollup {
    /// Creates a new stage.
    pub fn new(rollup_client: String) -> Self {
        Self { rollup_client }
    }
}