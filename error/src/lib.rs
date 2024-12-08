use kube;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    KubeError { source: kube::Error },
}
