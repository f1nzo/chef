enum Units {
    Volume(VolumeUnits),
    Weight(WeightUnits),
}

enum VolumeUnits {
    Metric(MetricVolumeUnits),
    Imperial(ImperialVolumeUnits),
    Units(f64),
}

enum WeightUnits {
    Metric(MetricWeightUnits),
    Imperial(ImperialWeightUnits),
}

// Metric volume units
enum MetricVolumeUnits {
    Milliliters(f64),
    Liters(f64),
}

// Imperial volume units
enum ImperialVolumeUnits {
    Teaspoons(f64),
    Tablespoons(f64),
    FluidOunces(f64),
    Cups(f64),
    Pints(f64),
    Quarts(f64),
    Gallons(f64),
}

// Metric weight units
enum MetricWeightUnits {
    Milligrams(f64),
    Grams(f64),
    Kilograms(f64),
}

// Imperial weight units
enum ImperialWeightUnits {
    Ounces(f64),
    Pounds(f64),
}

// Example of using these enums
fn example_usage() {
    // Represent 250 milliliters (metric volume)
    let metric_volume = KitchenUnits::Volume(VolumeUnits::Metric(MetricVolumeUnits::Milliliters(250.0)));
    // Represent 1 pound (imperial weight)
    let imperial_weight = KitchenUnits::Weight(WeightUnits::Imperial(ImperialWeightUnits::Pounds(1.0)));

    // Display examples
    display_kitchen_unit(metric_volume);
    display_kitchen_unit(imperial_weight);
}

// Mock function to display how a unit might be used
fn display_kitchen_unit(unit: KitchenUnits) {
    match unit {
        KitchenUnits::Volume(v) => match v {
            VolumeUnits::Metric(m) => match m {
                MetricVolumeUnits::Milliliters(ml) => println!("{} ml", ml),
                MetricVolumeUnits::Liters(l) => println!("{} l", l),
            },
            VolumeUnits::Imperial(i) => match i {
                ImperialVolumeUnits::Teaspoons(tsp) => println!("{} tsp", tsp),
                ImperialVolumeUnits::Tablespoons(tbsp) => println!("{} tbsp", tbsp),
                _ => (),
            },
        },
        KitchenUnits::Weight(w) => match w {
            WeightUnits::Metric(m) => match m {
                MetricWeightUnits::Grams(g) => println!("{} g", g),
                MetricWeightUnits::Kilograms(kg) => println!("{} kg", kg),
                _ => (),
            },
            WeightUnits::Imperial(i) => match i {
                ImperialWeightUnits::Ounces(oz) => println!("{} oz", oz),
                ImperialWeightUnits::Pounds(lb) => println!("{} lb", lb),
                _ => (),
            },
        },
    }
}