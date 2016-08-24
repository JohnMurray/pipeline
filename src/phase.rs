extern crate rustc_serialize;
use rustc_serialize::json::Json

/// Defines a Phase as a set of pure functions.
trait Phase {
    fn can_process(input: Json) -> bool;

    fn process(input: Json) -> Json;
}

/// A requirement definition for Phase input. Describes a single field that
/// is used by the Phase.
struct PhaseRequirement {
    field: String,
    _type: String,
    required: bool,
}

enum PhaseState {
    /// Things are as expected and should be operating normally for the phase
    ActiveOk,
    /// Things are operating as expected, but the phase is reporting that it is
    /// at or over capacity. Indicates we need to pause some pipelines at this
    /// phase and throttle to its capacity.
    ActiveOverCapacity,
    /// Phase is not responding to reqeusts and/or timing out. We should back off
    /// on using this phase and throttle back throughput.
    InActiveNonResponsive,
    /// Phase has indicated that it is unavailable. Reasons for this belong to the
    /// phase, but may include things such as maintenance or upgrades of the phase
    /// processor.
    InactiveUnavailable,
}

/// Data associated with Phase. This is data that is persistent for the life of the
/// phase and is not associated with a single invocation of a Phase.
struct PhaseData {
    requirements: Vec<PhaseRequirement>,
    state: PhaseState,
}
