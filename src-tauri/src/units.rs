use std::string::String;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum KitchenUnit {
    Milliliters,
    Liters,
    Teaspoons,
    Tablespoons,
    FluidOunces,
    Cups,
    Pints,
    Quarts,
    Gallons,
    Milligrams,
    Grams,
    Kilograms,
    Ounces,
    Pounds,
    Units,
}

impl KitchenUnit {
    pub fn from_string(s: String) -> Option<Self> {
        let trimed_string = s.trim().to_lowercase();
        let unit_string = trimed_string.trim_end_matches('s');

        match unit_string {
            "ml" | "milliliter" => Some(KitchenUnit::Milliliters),
            "l" | "liter" => Some(KitchenUnit::Liters),
            "tsp" | "teaspoon" => Some(KitchenUnit::Teaspoons),
            "tbsp" | "tablespoon" => Some(KitchenUnit::Tablespoons),
            "fl oz" | "fluid ounce" => Some(KitchenUnit::FluidOunces),
            "cup" => Some(KitchenUnit::Cups),
            "pt" | "pint" => Some(KitchenUnit::Pints),
            "qt" | "quart" => Some(KitchenUnit::Quarts),
            "gal" | "gallon" => Some(KitchenUnit::Gallons),
            "mg" | "milligram" => Some(KitchenUnit::Milligrams),
            "g" | "gram" => Some(KitchenUnit::Grams),
            "kg" | "kilogram" => Some(KitchenUnit::Kilograms),
            "oz" | "ounce" => Some(KitchenUnit::Ounces),
            "lb" | "pound" => Some(KitchenUnit::Pounds),
            "unit" => Some(KitchenUnit::Units),
            _ => None,
        }
    }

    pub fn to_string(&self, multiple: bool) -> String {
        match self {
            KitchenUnit::Milliliters => if multiple { "milliliters".to_string() } else { "milliliter".to_string() },
            KitchenUnit::Liters => if multiple { "liters".to_string() } else { "liter".to_string() },
            KitchenUnit::Teaspoons => if multiple { "teaspoons".to_string() } else { "teaspoon".to_string() },
            KitchenUnit::Tablespoons => if multiple { "tablespoons".to_string() } else { "tablespoon".to_string() },
            KitchenUnit::FluidOunces => if multiple { "fluid ounces".to_string() } else { "fluid ounce".to_string() },
            KitchenUnit::Cups => if multiple { "cups".to_string() } else { "cup".to_string() },
            KitchenUnit::Pints => if multiple { "pints".to_string() } else { "pint".to_string() },
            KitchenUnit::Quarts => if multiple { "quarts".to_string() } else { "quart".to_string() },
            KitchenUnit::Gallons => if multiple { "gallons".to_string() } else { "gallon".to_string() },
            KitchenUnit::Milligrams => if multiple { "milligrams".to_string() } else { "milligram".to_string() },
            KitchenUnit::Grams => if multiple { "grams".to_string() } else { "gram".to_string() },
            KitchenUnit::Kilograms => if multiple { "kilograms".to_string() } else { "kilogram".to_string() },
            KitchenUnit::Ounces => if multiple { "ounces".to_string() } else { "ounce".to_string() },
            KitchenUnit::Pounds => if multiple { "pounds".to_string() } else { "pound".to_string() },
            KitchenUnit::Units => if multiple { "units".to_string() } else { "unit".to_string() },
        }
    }
}