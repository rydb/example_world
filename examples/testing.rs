use std::fmt::Display;

use bevy::{prelude::*, utils::HashMap};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<HelloCounter>()
    .init_resource::<ShoppingCart>()
    .add_systems(Startup, add_to_party("bob", "ice cream"))
    .add_systems(Startup, add_to_party("joe", "spaghetti"))
    .add_systems(Startup, add_to_party("sam", "enchiladas"))
    .run();
}

#[derive(Resource, Default)]
pub struct HelloCounter(u8);

#[derive(Resource, Default)]
pub struct ShoppingCart(HashMap<String, String>);

/// greet geust at the door, and add their favorite food to the shopping cart.
pub fn add_to_party(name: impl Into<String> + Display + Clone, favorite_food: impl Into<String> + Display + Clone) -> 
    impl Fn(ResMut<HelloCounter>, ResMut<ShoppingCart>)
{
    move |mut counter, mut shopping_cart| {
        println!("hello, {:#}!", name);
        counter.0 += 1;

        println!("greeting total now: {:#}", counter.0);

        shopping_cart.0.insert(name.clone().into(), favorite_food.clone().into());

        println!("shopping cart now: {:#?}", shopping_cart.0);
    }
}