use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyConfigSystemSet {
    OfflineEditable,
    LiveEditable
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyCompositingSystemSet {
    Previews,
    Programs
}

