use skyrim_alchemy_db::*;
use wasm_bindgen::prelude::*;

#[allow(unused)]
fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

#[wasm_bindgen(getter_with_clone)]
pub struct AlchemyError {
    pub message: String,
}

impl From<String> for AlchemyError {
    fn from(message: String) -> Self {
        AlchemyError { message }
    }
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
    inventory: skyrim_alchemy_db::Inventory,
}

#[wasm_bindgen(getter_with_clone)]
pub struct Recipe {
    pub effect: String,
    pub value: u32,
    pub ingredients: Vec<String>,
    pub kind: RecipeKind,
}

#[wasm_bindgen]
pub struct InventoryAlchemy {
    alchemy_data: AlchemyData,
}

#[wasm_bindgen]
impl InventoryAlchemy {
    #[wasm_bindgen]
    pub fn load() -> Result<InventoryAlchemy, AlchemyError> {
        let alchemy_data =
            AlchemyData::load_builtin().map_err(|e| AlchemyError::from(e.to_string()))?;
        Ok(Self { alchemy_data })
    }
}

#[wasm_bindgen]
impl Inventory {
    #[wasm_bindgen]
    pub fn parse(text: &str) -> Result<Inventory, AlchemyError> {
        let inventory = parse_inventory(text);
        let items = inventory.items.iter().map(|i| i.name.clone()).collect();
        let inventory = Inventory { items, inventory };
        Ok(inventory)
    }

    pub fn recipes(&self, alchemy: InventoryAlchemy, search: String) -> Result<Vec<Recipe>, AlchemyError> {
        let alchemy = AlchemyDataIndexes::new(&alchemy.alchemy_data);
        let search = search.trim();
        let search = if search.len() > 0 {
            SearchTerm::Any(search.into())
        } else {
            SearchTerm::None
        };

        let recipes = alchemy
            .find_recipes(&self.inventory, &search)
            .into_iter()
            .map(|r| {
                let mut ingredients = r.ingredients.clone();
                ingredients.sort();
                Recipe {
                    effect: r.effect,
                    value: r.value,
                    ingredients,
                    kind: r.kind.into(),
                }
            })
            .collect();
        Ok(recipes)
    }
}
