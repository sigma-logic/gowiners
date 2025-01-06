pub mod cli;
mod eda;
mod pipeline;
mod project;

pub use eda::GowinEda;
pub use pipeline::{Pipeline, PipelineError, TclCommand, commands, EvaluationError};
pub use project::{Device, Hdl, MaybeList, Project};
