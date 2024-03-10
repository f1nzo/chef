// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    name: String,
    image: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
    cuisine: String,
    prep_time: u32,
    cook_time: u32,
    serves: u32,
}

#[tauri::command]
fn load_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            name: "Ramen".to_string(),
            image: "https://media.istockphoto.com/id/511793034/photo/japanese-ramen-soup-with-chicken-egg-chives-and-sprout.jpg?b=1&s=612x612&w=0&k=20&c=SpW8Op5RozB2pGVn6EJcZKd_xwND7zL923RJkvVx5IE=".to_string(),
            ingredients: vec![
                "1 1/2 cups all-purpose flour".to_string(),
                "3 1/2 teaspoons baking powder".to_string(),
                "1 teaspoon salt".to_string(),
                "1 tablespoon white sugar".to_string(),
                "1 1/4 cups milk".to_string(),
                "1 egg".to_string(),
                "3 tablespoons butter, melted".to_string(),
            ],
            instructions: vec![
                "In a large bowl, sift together the flour, baking powder, salt and sugar.".to_string(),
                "Make a well in the center and pour in the milk, egg and melted butter; mix until smooth.".to_string(),
                "Heat a lightly oiled griddle or frying pan over medium high heat.".to_string(),
                "Pour or scoop the batter onto the griddle, using approximately 1/4 cup for each pancake.".to_string(),
                "Brown on both sides and serve hot.".to_string(),
            ],
            cuisine: "American".to_string(),
            prep_time: 5,
            cook_time: 15,
            serves: 4,
        }
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_recipes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
