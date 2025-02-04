use clap::{ArgAction, Parser};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::common::{Command, CommonOptions};
use crate::{heading, CargoOptions, CargoOptionsExt};

/// Compile a local package and all of its dependencies
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help clean` for more detailed information."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Clean {
    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonOptions,

    #[arg(long = "doc")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub doc: bool,

    #[arg(short = 'n', long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub dry_run: bool,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH", help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_path: Option<PathBuf>,

    /// Clean artifacts in release mode, with optimizations
    #[arg(short = 'r', long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub release: bool,

    /// Package to clean (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub packages: Vec<String>,
}

impl CargoOptionsExt for Clean {
    /// Build a `cargo clean` options
    fn options(&self) -> CargoOptions {
        let mut cmd = CommonOptions::cargo_options();

        self.common.apply_options(&mut cmd);

        if let Some(path) = self.manifest_path.as_ref() {
            cmd.arg("--manifest-path").arg(path);
        }

        if self.doc {
            cmd.arg("--doc");
        }

        if self.dry_run {
            cmd.arg("--dry-run");
        }

        if self.release {
            cmd.arg("--release");
        }

        for pkg in &self.packages {
            cmd.arg("--package").arg(pkg);
        }

        cmd
    }
}

impl Clean {
    /// Build a `cargo clean` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("clean");

        cmd.args(self.options());

        cmd
    }
}

impl Deref for Clean {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Clean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Clean;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Clean as CommandFactory>::command().debug_assert()
    }
}
