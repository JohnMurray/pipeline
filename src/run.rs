extern crate rustc_serialize;
use rustc_serialize::json::Json

/// RunConfiguration
///   This describes the build definitions or set of phases that a run will
///   go through. This can generally be thought of as the "configuration" for any given
///   run and a representation of what is provided in the `pipeline.yaml` file.
struct RunConfiguration {
    phaseNames: Vec[String],
}

/// Run
///   Represents a "run through" of hte pipeline, meaning an instance in time when
///   the RunConfiguration was executed.
struct Run {
    config: RunConfiguration,
    phaseResults: Vec[Json],
}
