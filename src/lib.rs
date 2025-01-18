mod build;
mod check;
mod clippy;
mod common;
mod doc;
mod install;
mod metadata;
mod run;
mod rustc;
mod test;

pub mod heading {
    pub const PACKAGE_SELECTION: &str = "Package Selection";
    pub const TARGET_SELECTION: &str = "Target Selection";
    pub const FEATURE_SELECTION: &str = "Feature Selection";
    pub const COMPILATION_OPTIONS: &str = "Compilation Options";
    pub const MANIFEST_OPTIONS: &str = "Manifest Options";
}

pub fn styles() -> clap::builder::Styles {
    use anstyle::{AnsiColor, Effects};

    clap::builder::styling::Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
        .valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
}

// Specify crate to satisfy naming overlap w/ rustc clippy
pub use crate::clippy::Clippy;
pub use build::Build;
pub use check::Check;
pub use common::CommonOptions;
pub use doc::Doc;
pub use install::Install;
pub use metadata::Metadata;
pub use run::Run;
pub use rustc::Rustc;
use std::ffi::OsStr;
use std::ffi::OsString;
pub use test::Test;

pub trait CargoOptionsExt {
    fn options(&self) -> CargoOptions;
}

#[derive(Clone, Debug, Default)]
pub struct CargoOptions {
    inner: Vec<OsString>,
}

impl CargoOptions {
    pub fn arg<Arg: AsRef<OsStr>>(&mut self, arg: Arg) -> &mut Self {
        self.inner.push(arg.as_ref().to_os_string());
        self
    }

    pub fn args<Args: IntoIterator<Item = Arg>, Arg: AsRef<OsStr>>(
        &mut self,
        args: Args,
    ) -> &mut Self {
        self.inner
            .extend(args.into_iter().map(|arg| arg.as_ref().to_os_string()));
        self
    }

    pub fn into_string(self) -> Option<String> {
        self.into_iter()
            .map(|os| os.into_string().ok())
            .collect::<Option<Vec<String>>>()
            .map(|v| v.join(" "))
    }
}

impl std::iter::IntoIterator for CargoOptions {
    type Item = OsString;
    type IntoIter = std::vec::IntoIter<OsString>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
