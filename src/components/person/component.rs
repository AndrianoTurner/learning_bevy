use bevy::prelude::{Component, Query, Commands, With};
#[derive(Component)]
pub struct Person;
#[derive(Component)]
pub struct Name(pub String);
