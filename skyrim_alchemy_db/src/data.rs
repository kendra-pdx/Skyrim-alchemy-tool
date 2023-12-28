use std::path::Path;

use anyhow::Result;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub(crate) struct Value(pub u32);

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Weight(BigDecimal);

#[allow(unused)]
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Ingredient {
    pub(crate) name: String,
    pub(crate) effects: Vec<String>,
    pub(crate) obtained: String,
    pub(crate) value: Value,
    pub(crate) weight: Weight,
}

#[allow(unused)]
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Effect {
    pub(crate) effect: String,
    pub(crate) value: Value,
}

#[allow(unused)]
pub struct AlchemyData {
    ingredients: Vec<Ingredient>,
    effects: Vec<Effect>,
}

impl AlchemyData {
    fn load(ingredients: &str, effects: &str) -> Result<AlchemyData> {
        let ingredients = serde_json::from_str::<Vec<Ingredient>>(ingredients)?;
        let effects = serde_json::from_str::<Vec<Effect>>(effects)?;
        
        Ok(AlchemyData {
            ingredients,
            effects
        })
    }

    pub fn load_paths(ingredients_path: &Path, effects_path: &Path) -> Result<AlchemyData> {
        let ingredients = read_to_string(ingredients_path)?;
        let effects = read_to_string(effects_path)?;
        AlchemyData::load(&ingredients, &effects)
    }

    pub fn load_builtin() -> Result<AlchemyData> {
        let ingredients = include_str!("../ingredients.json");
        let effects = include_str!("../effects.json");
        AlchemyData::load(ingredients, effects)
    }

    pub(crate) fn find_ingredient_by_name(&self, name: &str) -> Option<&Ingredient> {
        self.ingredients.iter().find(|i| i.name.as_str() == name)
    }

    pub(crate) fn find_effect_by_name(&self, name: &str) -> Option<&Effect> {
        self.effects.iter().find(|e| e.effect == name)
    }
}