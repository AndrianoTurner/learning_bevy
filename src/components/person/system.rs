use bevy::prelude::{Component, Query, Commands, With};
use super::{component::{Name,Person}};
pub fn add_people(mut commands : Commands){
    commands.spawn((Person,Name("Andriano Turner".into())));
    commands.spawn((Person,Name("Andriano 2Turner".into())));
    commands.spawn((Person,Name("Andriano 1Turner".into())));
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
