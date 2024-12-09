use kube;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("catch kube error: {}", source))]
    KubeError { source: kube::Error },
}
