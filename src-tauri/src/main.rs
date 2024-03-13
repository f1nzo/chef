// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod units;

use units::KitchenUnit;
use serde::{Deserialize, Serialize};
use tauri::{api::path::{BaseDirectory, resolve_path}, Env};
use std::{fs::{self, File}, io::Write};
use std::path::PathBuf;


#[derive(Debug, Serialize, Deserialize)]
struct Ingredient {
    name: String, // Name of the ingredient
    quantity: f64, // Quantity of the ingredient
    unit: String, // Unit of the ingredient
}

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    id: u32, // Unique identifier for the recipe
    name: String, // Name of the recipe
    image: String, // Image of the recipe
    ingredients: Vec<Ingredient>, // Ingredients of the recipe
    instructions: Vec<String>, // Instructions for the recipe
    cuisine: String, // Cuisine of the recipe
    prep_time: u32, // Preparation time in minutes
    cook_time: u32, // Cooking time in minutes
    serves: u32, // Number of servings
}

#[tauri::command]
fn load_recipes() -> Vec<Recipe> {
    let context = tauri::generate_context!();
    let path: PathBuf = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "recipes.json",
        Some(BaseDirectory::AppData))
        .expect("failed to resolve path");

    // Ensure all parent directories exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create directories");
    }

    // If file does not exist, create it
    if !path.exists() {
        let mut file: File = File::create(&path).expect("failed to create file");
        file.write_all("[]".as_bytes()).expect("failed to write '[]' to file");
    }
    let recipes: String = fs::read_to_string(&path).expect("failed to read file");
    serde_json::from_str(&recipes).expect("failed to parse recipes")
}

#[tauri::command]
fn save_recipes(recipes: Vec<Recipe>) {
    let context = tauri::generate_context!();
    let path: PathBuf = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "recipes.json",
        Some(BaseDirectory::AppData))
        .expect("failed to resolve path");

    let recipes: String = serde_json::to_string(&recipes).expect("failed to serialize recipes");
    fs::write(&path, recipes).expect("failed to write recipes to file");
}

#[tauri::command]
fn add_recipe(mut recipe: Recipe) {
    let mut recipes: Vec<Recipe> = load_recipes();

    let new_id = recipes.iter().map(|r| r.id).max().unwrap_or(0) + 1;
    recipe.id = new_id;

    recipes.push(recipe);
    save_recipes(recipes);
}

#[tauri::command]
fn delete_recipe(recipe_id: u32) {
    let mut recipes: Vec<Recipe> = load_recipes();

    recipes.retain(|r| r.id != recipe_id);
    save_recipes(recipes);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_recipes, save_recipes, add_recipe, delete_recipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
