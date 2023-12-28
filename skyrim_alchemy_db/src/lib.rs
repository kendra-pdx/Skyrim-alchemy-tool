mod data;
use std::collections::{HashMap, HashSet};

pub use data::AlchemyData;
use lazy_static::lazy_static;

use crate::data::{Effect, Ingredient};

#[derive(Clone, Copy)]
pub enum RecipeKind {
    Restore,
    Fortify,
    Harm,
    Other,
}

impl From<&str> for RecipeKind {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref KEYWORDS: Vec<(&'static str, RecipeKind)> = vec![
                ("Restore", RecipeKind::Restore),
                ("Regenerate", RecipeKind::Restore),
                ("Resist", RecipeKind::Restore),
                ("Fortify", RecipeKind::Fortify),
                ("Damage", RecipeKind::Harm),
                ("Ravage", RecipeKind::Harm),
                ("Weakness", RecipeKind::Harm),
            ];
        }

        KEYWORDS
            .iter()
            .find(|(k, _)| value.contains(k))
            .map(|(_, t)| *t)
            .unwrap_or(RecipeKind::Other)
    }
}

#[allow(unused)]
pub struct Recipe {
    pub effect: String,
    pub ingredients: Vec<String>,
    pub value: u32,
    pub kind: RecipeKind
}

pub trait Search {
    fn find_recipes(&self, inventory: &[&str]) -> Vec<Recipe>;
}

impl Search for AlchemyData {
    fn find_recipes(&self, inventory: &[&str]) -> Vec<Recipe> {
        let mut effect_ingredients: HashMap<&Effect, HashSet<&Ingredient>> = HashMap::default();
        let ingredients = inventory
            .iter()
            .flat_map(|i| self.find_ingredient_by_name(i));
        for ingredient in ingredients {
            let effects = ingredient
                .effects
                .iter()
                .flat_map(|e| self.find_effect_by_name(e));

            for effect in effects {
                let is = effect_ingredients.entry(effect).or_default();
                is.insert(ingredient);
            }
        }

        let mut recipes = effect_ingredients
            .into_iter()
            .filter(|(_, ingredients)| ingredients.len() >= 2)
            .map(|(effect, ingredients)| Recipe {
                effect: effect.effect.clone(),
                ingredients: ingredients.iter().map(|i| i.name.clone()).collect(),
                value: effect.value.0,
                kind: RecipeKind::from(effect.effect.as_str())
            })
            .collect::<Vec<_>>();

        recipes.sort_by(|a, b| b.value.cmp(&a.value));

        recipes
    }
}
