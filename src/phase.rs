extern crate rustc_serialize;
use rustc_serialize::json::{Json, Null}

/// Data associated with a Phase
struct Phase {
    name: String,
    requirements: Vec<PhaseRequirement>,
    state: PhaseState,
}

/// Defines a Phase as a set of pure functions.
impl Phase {
    fn can_process(input: Json) -> bool {
        // TODO: implement method
        false
    }

    fn process(input: Json) -> Json {
        // TODO: implement method
        Null
    }
}

/// A requirement definition for Phase input. Describes a single field that
/// is used by the Phase.
struct PhaseRequirement {
    field: String,
    _type: FieldType,
    required: bool,
}

enum Fieldtype {
    String,
    Bool,
    Numeric,
    // TODO: figure out how/if we need to define structural / array types
}

enum PhaseState {
    Active,
    Inactive,
}
