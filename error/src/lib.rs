use kube;
use serde_yaml;
use snafu::prelude::*;
use std::io;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(context(false))]
    KubeError {
        source: kube::Error,
    },

    #[snafu(context(false))]
    IOError {
        source: io::Error,
    },

    #[snafu(context(false))]
    DecodeError {
        source: serde_yaml::Error,
    },

    TemplateInitError,

    MissingObjectKey,
}
