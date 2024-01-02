mod data;

use data::AlchemyDataFind;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use std::collections::{HashMap, HashSet};

pub use data::{AlchemyData, AlchemyDataIndexes};
use lazy_static::lazy_static;

use crate::data::{Effect, Ingredient};

pub struct InventoryItem {
    pub name: String,
}

pub struct Inventory {
    pub items: Vec<InventoryItem>,
}

pub fn parse_inventory(inventory: &str) -> Inventory {
    let items = inventory
        .split('\n')
        .map(|i| i.trim_matches(|c: char| c.is_numeric() || c == '(' || c == ')'))
        .map(|i| i.trim())
        .filter(|i| i.trim().len() > 0)
        .map(|s| InventoryItem {
            name: s.to_string(),
        })
        .collect::<Vec<_>>();

    Inventory { items }
}

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
    pub kind: RecipeKind,
}

pub enum SearchTerm {
    None,
    Any(String),
}

pub trait Search {
    fn find_recipes(&self, inventory: &Inventory, search: &SearchTerm) -> Vec<Recipe>;
}

impl<'a> Search for AlchemyDataIndexes<'a> {
    fn find_recipes(&self, inventory: &Inventory, search: &SearchTerm) -> Vec<Recipe> {
        let mut effect_ingredients: HashMap<&Effect, HashSet<&Ingredient>> = HashMap::default();

        let ingredients = inventory
            .items
            .iter()
            .map(|i| i.name.as_str())
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
            .filter(|(effect, ingredients)| match search {
                SearchTerm::None => true,
                SearchTerm::Any(find) => {
                    let find = find.to_lowercase();
                    effect.effect.to_lowercase().contains(&find)
                        || ingredients
                            .iter()
                            .any(|i| i.name.to_lowercase().contains(&find))
                }
            })
            .map(|(effect, ingredients)| Recipe {
                effect: effect.effect.clone(),
                ingredients: ingredients.iter().map(|i| i.name.clone()).collect(),
                value: effect.value.0,
                kind: RecipeKind::from(effect.effect.as_str()),
            })
            .collect::<Vec<_>>();

        recipes.sort_by(|a, b| {
            if a.value == b.value {
                a.effect.cmp(&b.effect)
            } else {
                b.value.cmp(&a.value)
            }
        });

        recipes
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_inventory;

    #[test]
    fn test_parse_inventory() {
        let inventory = parse_inventory(
            "
            a
            b (1)
            c (10)
        ",
        );
        assert_eq!(
            vec!["a", "b", "c"],
            inventory.items.iter().map(|i| &i.name).collect::<Vec<_>>()
        )
    }
}
