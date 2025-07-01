use bevy::prelude::*;

// See https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.SystemSet.html (incomplete in 0.16.1)
// or https://dev-docs.bevy.org/bevy/ecs/schedule/trait.SystemSet.html#adding-systems-to-system-sets

/* TODO design considerations
  - some config are Path and str, that don't have Sized trait, getting them in ECS is questionnable.
    but some dynamic components will need strings (a textbox...) and could be reread at each frame
  - We can lower previews render framerate if system overload occurs...
    but maybe adding all displayable-related systems twice is non-sense ?
*/ 
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

