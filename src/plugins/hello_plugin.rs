use bevy::prelude::*;
pub struct HelloPlugin;
use crate::components::{person::system::{add_people,greet_people}};
impl Plugin for HelloPlugin{
    fn build(&self,app : &mut App){
        app.add_startup_system(add_people)
        .add_system(greet_people);
    }
}