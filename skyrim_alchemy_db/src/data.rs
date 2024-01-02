use std::{collections::HashMap, path::Path};

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
            effects,
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
}

pub(crate) trait AlchemyDataFind {
    fn find_ingredient_by_name(&self, name: &str) -> Option<&Ingredient>;
    fn find_effect_by_name(&self, name: &str) -> Option<&Effect>;
}

pub struct AlchemyDataIndexes<'a> {
    ingredients_by_name: HashMap<&'a str, &'a Ingredient>,
    effects_by_name: HashMap<&'a str, &'a Effect>
}

impl<'a> AlchemyDataIndexes<'a> {
    pub fn new(alchemy_data: &'a AlchemyData) -> AlchemyDataIndexes<'a> {
        let ingredients_by_name = alchemy_data
            .ingredients
            .iter()
            .map(|i| (i.name.as_str(), i))
            .collect();
        
        let effects_by_name = alchemy_data
            .effects
            .iter()
            .map(|i| (i.effect.as_str(), i))
            .collect();
        
        Self {
            ingredients_by_name,
            effects_by_name,
        }
    }
}

impl<'a> AlchemyDataFind for AlchemyDataIndexes<'a> {
    fn find_ingredient_by_name(&self, name: &str) -> Option<&Ingredient> {
        self.ingredients_by_name.get(name).copied()
    }

    fn find_effect_by_name(&self, name: &str) -> Option<&Effect> {
        self.effects_by_name.get(name).copied()
    }
}