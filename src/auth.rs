use std::collections::HashMap;

// TODO: do these need to impl partialeq/hash/key?
pub enum Group {
    Public,
    Named(String)
}

pub enum Access {
    Read,
    ReadWrite
}

trait Resource {
    fn access() -> HashMap<Group, Access>;
}

trait Accessor {
    fn groups() -> Vec<Group>;
}

// TODO: proc macros?
