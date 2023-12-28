use skyrim_alchemy_db::{parse_inventory, AlchemyData, Search};
use wasm_bindgen::prelude::*;

#[allow(unused)]
fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

#[wasm_bindgen]
pub enum Errors {
    ParseError,
    RecipesError,
    NotImplemented,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum RecipeKind {
    Restore,
    Fortify,
    Harm,
    Other,
}

impl From<skyrim_alchemy_db::RecipeKind> for RecipeKind {
    fn from(value: skyrim_alchemy_db::RecipeKind) -> Self {
        match value {
            skyrim_alchemy_db::RecipeKind::Restore => RecipeKind::Restore,
            skyrim_alchemy_db::RecipeKind::Fortify => RecipeKind::Fortify,
            skyrim_alchemy_db::RecipeKind::Harm => RecipeKind::Harm,
            skyrim_alchemy_db::RecipeKind::Other => RecipeKind::Other,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Inventory {
    pub items: Vec<String>,
    inventory: skyrim_alchemy_db::Inventory
}

#[wasm_bindgen(getter_with_clone)]
pub struct Recipe {
    pub effect: String,
    pub value: u32,
    pub ingredients: Vec<String>,
    pub kind: RecipeKind,
}

#[wasm_bindgen]
impl Inventory {
    #[wasm_bindgen]
    pub fn parse(text: &str) -> Result<Inventory, Errors> {
        let inventory = parse_inventory(text);
        let items = inventory.items.iter().map(|i| i.name.clone()).collect();
        let inventory = Inventory { items, inventory };
        Ok(inventory)
    }

    pub fn recipes(&self) -> Result<Vec<Recipe>, Errors> {
        let alchemy = AlchemyData::load_builtin().map_err(|_| Errors::RecipesError)?;
        let recipes = alchemy.find_recipes(&self.inventory).into_iter().map(|r| {
            Recipe {
                effect: r.effect,
                value: r.value,
                ingredients: r.ingredients.clone(),
                kind: r.kind.into()
            }
        })
        .collect();
        Ok(recipes)
    }
}
