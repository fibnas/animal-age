use clap::Parser;
use console::Term;
use serde::Serialize;
use std::process::exit;
use strsim::levenshtein;
use thiserror::Error;

mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const CYAN: &str = "\x1b[36m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
}

/// CLI tool to convert animal years to human years and show lifespan progress.
#[derive(Parser)]
#[command(
    name = "animal-age",
    version = "3.0",
    about = "Convert animal age to human years & show colorful lifespan comparisons",
    after_help = "Examples:\n\
                  \tanimal-age -t cat -a 3\n\
                  \tanimal-age --type small_dog --age 5\n\
                  \tanimal-age --list\n\
                  \tanimal-age -t horse -a 10 --json\n\
                  \tanimal-age -t cat,small_dog -a 3 --no-color\n"
)]
struct Args {
    /// Animal type (use --list to show valid options, supports comma-separated list)
    #[arg(short = 't', long = "type", value_name = "ANIMAL", value_delimiter = ',')]
    animal: Option<Vec<String>>,

    /// Age of the animal in real years
    #[arg(short = 'a', long = "age", value_name = "YEARS")]
    age: Option<f32>,

    /// Show supported animal types
    #[arg(long = "list")]
    list: bool,

    /// Output in JSON format
    #[arg(long = "json", help = "Output in JSON format")]
    json: bool,

    /// Disable colored output
    #[arg(long = "no-color", help = "Disable colored output")]
    no_color: bool,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Missing required arguments: --type and --age")]
    MissingArgs,
    #[error("Unknown animal type: {0}")]
    UnknownAnimal(String),
    #[error("Invalid age: {0}")]
    InvalidAge(String),
}

#[derive(Debug, Clone, Copy)]
enum Animal {
    SmallDog,
    MediumDog,
    BigDog,
    Cat,
    Horse,
    Pig,
    Parakeet,
    Snake,
    Goldfish,
    Rabbit,
    Hamster,
}

impl Animal {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "small_dog" => Some(Animal::SmallDog),
            "medium_dog" => Some(Animal::MediumDog),
            "big_dog" => Some(Animal::BigDog),
            "cat" => Some(Animal::Cat),
            "horse" => Some(Animal::Horse),
            "pig" => Some(Animal::Pig),
            "parakeet" => Some(Animal::Parakeet),
            "snake" => Some(Animal::Snake),
            "goldfish" => Some(Animal::Goldfish),
            "rabbit" => Some(Animal::Rabbit),
            "hamster" => Some(Animal::Hamster),
            _ => None,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Animal::SmallDog => "small_dog",
            Animal::MediumDog => "medium_dog",
            Animal::BigDog => "big_dog",
            Animal::Cat => "cat",
            Animal::Horse => "horse",
            Animal::Pig => "pig",
            Animal::Parakeet => "parakeet",
            Animal::Snake => "snake",
            Animal::Goldfish => "goldfish",
            Animal::Rabbit => "rabbit",
            Animal::Hamster => "hamster",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Animal::SmallDog => "Small dog (e.g., terrier)",
            Animal::MediumDog => "Medium dog (e.g., spaniel)",
            Animal::BigDog => "Large dog (e.g., retriever)",
            Animal::Cat => "Domestic cat",
            Animal::Horse => "Horse",
            Animal::Pig => "Pig",
            Animal::Parakeet => "Parakeet / budgie",
            Animal::Snake => "Common pet snake",
            Animal::Goldfish => "Goldfish",
            Animal::Rabbit => "Rabbit",
            Animal::Hamster => "Hamster",
        }
    }

    fn max_lifespan(&self) -> f32 {
        match self {
            Animal::SmallDog => 16.0,
            Animal::MediumDog => 14.0,
            Animal::BigDog => 10.0,
            Animal::Cat => 18.0,
            Animal::Horse => 30.0,
            Animal::Pig => 20.0,
            Animal::Parakeet => 10.0,
            Animal::Snake => 20.0,
            Animal::Goldfish => 15.0,
            Animal::Rabbit => 12.0,
            Animal::Hamster => 3.0,
        }
    }

    fn human_years(&self, age: f32) -> f32 {
        match self {
            Animal::SmallDog => if age <= 2.0 { age * 12.5 } else { 25.0 + (age - 2.0) * 4.5 },
            Animal::MediumDog => if age <= 2.0 { age * 10.5 } else { 21.0 + (age - 2.0) * 5.0 },
            Animal::BigDog => if age <= 2.0 { age * 9.0 } else { 18.0 + (age - 2.0) * 7.0 },
            Animal::Cat => if age <= 2.0 { age * 12.5 } else { 25.0 + (age - 2.0) * 4.0 },
            Animal::Horse => 6.5 + age * 4.0,
            Animal::Pig => age * 5.0,
            Animal::Parakeet => age * 5.0,
            Animal::Snake => age * 5.3,
            Animal::Goldfish => age * 5.0,
            Animal::Rabbit => if age <= 2.0 { age * 12.0 } else { 24.0 + (age - 2.0) * 4.0 },
            Animal::Hamster => age * 25.0,
        }
    }
}

fn main() {
    if let Err(err) = main_inner() {
        if let AppError::UnknownAnimal(ref animal) = err {
            if let Some(suggestion) = suggest_animal(animal) {
                eprintln!(
                    "Unknown animal type: {}. Did you mean '{}'?\nUse --list to view valid options.",
                    animal, suggestion
                );
            } else {
                eprintln!("Unknown animal type: {}\nUse --list to view valid options.", animal);
            }
        } else {
            eprintln!("Error: {}", err);
        }
        exit(1);
    }
}

fn main_inner() -> Result<(), AppError> {
    let args = Args::parse();

    if args.list {
        list_animals();
        return Ok(());
    }

    let (animals, age) = match (args.animal.as_ref(), args.age) {
        (Some(a), Some(y)) => (a, y),
        _ => return Err(AppError::MissingArgs),
    };

    if age < 0.0 {
        return Err(AppError::InvalidAge("Age cannot be negative".to_string()));
    }

    run_calc(animals.to_vec(), age, &args)?;
    Ok(())
}

fn list_animals() {
    println!("Available animals:\n");
    let animal_variants = [
        Animal::SmallDog,
        Animal::MediumDog,
        Animal::BigDog,
        Animal::Cat,
        Animal::Horse,
        Animal::Pig,
        Animal::Parakeet,
        Animal::Snake,
        Animal::Goldfish,
        Animal::Rabbit,
        Animal::Hamster,
    ];
    for animal in animal_variants {
        println!("  {:12} - {}", animal.key(), animal.description());
    }
}

fn run_calc(animals: Vec<String>, age: f32, args: &Args) -> Result<(), AppError> {
    for animal_str in animals {
        let animal_lower = animal_str.to_lowercase();
        let animal_type = Animal::from_str(&animal_lower)
            .ok_or_else(|| AppError::UnknownAnimal(animal_str.clone()))?;

        let animal_max = animal_type.max_lifespan();
        if age > animal_max * 1.5 {
            eprintln!(
                "Warning: Age {} exceeds typical {} lifespan of {} years.",
                age, animal_str, animal_max
            );
        }

        let human_age = (animal_type.human_years(age) * 10.0).round() / 10.0;

        if args.json {
            print_json(&animal_str, age, human_age, animal_max);
        } else {
            println!(
                "{} years old {} ≈ {:.1} human years\n",
                age, animal_str, human_age
            );
            println!("Life Progress:\n");
            show_lifespan_bars("Human", human_age.min(HUMAN_MAX), HUMAN_MAX, args.no_color);
            show_lifespan_bars(&animal_lower, age.min(animal_max), animal_max, args.no_color);
            println!();
        }
    }
    Ok(())
}

fn suggest_animal(input: &str) -> Option<String> {
    let animals = [
        "small_dog", "medium_dog", "big_dog", "cat", "horse", "pig",
        "parakeet", "snake", "goldfish", "rabbit", "hamster",
    ];
    animals
        .iter()
        .min_by_key(|&&animal| levenshtein(input, animal))
        .filter(|&&animal| levenshtein(input, animal) < 3)
        .map(|&animal| animal.to_string())
}

const HUMAN_MAX: f32 = 80.0;

fn show_lifespan_bars(label: &str, age: f32, max: f32, no_color: bool) {
    let term = Term::stdout();
    let term_width = term.size().1 as usize;
    let total_width = (term_width.saturating_sub(15)).min(50);
    let pct = age / max;
    let filled = (pct * total_width as f32) as usize;
    let empty = total_width - filled;

    let color_code = if no_color {
        ""
    } else if pct >= 0.8 {
        color::RED
    } else if pct >= 0.6 {
        color::YELLOW
    } else {
        color::CYAN
    };

    let bar = format!(
        "{}{} {}{}",
        color_code,
        "=".repeat(filled),
        " ".repeat(empty),
        if no_color { "" } else { color::RESET }
    );

    println!("{:10} |{}| {:>3.0}%", label, bar, pct * 100.0);
}

#[derive(Serialize)]
struct Output {
    animal: String,
    age: f32,
    human_age: f32,
    animal_max_lifespan: f32,
    human_max_lifespan: f32,
    animal_progress: f32,
    human_progress: f32,
}

fn print_json(animal: &str, age: f32, human_age: f32, animal_max: f32) {
    let output = Output {
        animal: animal.to_string(),
        age,
        human_age,
        animal_max_lifespan: animal_max,
        human_max_lifespan: HUMAN_MAX,
        animal_progress: age / animal_max,
        human_progress: human_age / HUMAN_MAX,
    };
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_age_conversion() {
        let cat = Animal::Cat;
        assert_eq!(cat.human_years(1.0), 12.5);
        assert_eq!(cat.human_years(3.0), 29.0);
    }

    #[test]
    fn test_max_lifespan() {
        assert_eq!(Animal::SmallDog.max_lifespan(), 16.0);
        assert_eq!(Animal::Hamster.max_lifespan(), 3.0);
    }

    #[test]
    fn test_animal_from_str() {
        assert!(Animal::from_str("cat").is_some());
        assert!(Animal::from_str("CAT").is_some());
        assert!(Animal::from_str("invalid").is_none());
    }
}
