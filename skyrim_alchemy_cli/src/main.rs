use std::{fmt::Display, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use skyrim_alchemy_db::*;
use std::io::read_to_string;

#[derive(Parser)]
struct CliArgs {
    #[arg(short)]
    ingredients_data_path: Option<PathBuf>,

    #[arg(short)]
    effects_data_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let alchemy = match (args.ingredients_data_path, args.effects_data_path) {
        (Some(ingredients_path), Some(effects_path)) => Ok(AlchemyData::load_paths(
            ingredients_path.as_path(),
            effects_path.as_path(),
        )?),
        (None, None) => Ok(AlchemyData::load_builtin()?),
        _ => Err(anyhow!("-i and -e must be used together")),
    }?;

    let alchemy = AlchemyDataIndexes::new(&alchemy);

    let inventory = read_to_string(std::io::stdin())?;
    let inventory = parse_inventory(inventory.as_str());

    let recipes = alchemy.find_recipes(&inventory, &SearchTerm::None);

    struct RecipeDisplay<'a>(&'a Recipe);
    impl<'a> Display for RecipeDisplay<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let name = match self.0.kind {
                RecipeKind::Fortify => self.0.effect.blue(),
                RecipeKind::Restore => self.0.effect.green(),
                RecipeKind::Harm => self.0.effect.red(),
                RecipeKind::Other => self.0.effect.white(),
            };
            let ingredients = self.0.ingredients.join(", ");
            let value = self.0.value.to_string() + "G";
            write!(f, "{} ({}): {}", name, value.yellow(), ingredients)
        }
    }
    for recipe in recipes.iter().map(RecipeDisplay) {
        println!("{recipe}");
    }

    Ok(())
}
