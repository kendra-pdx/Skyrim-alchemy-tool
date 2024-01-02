<script setup lang="ts">
import { inject, ref, watch } from 'vue';
import skyrim_alchemy_wasm_init, * as alchemy from 'skyrim_alchemy_wasm'

skyrim_alchemy_wasm_init().then(() => {
    console.log("alchemy wasm initialized")
})

let search = ref<string>("")
let pastebox = ref<string>("")
let recipes = ref<alchemy.Recipe[]>([])
let inventoryItems = ref<string[]>([])
let showInventoryParse = ref<boolean>(false)

function load_recipes(inventory_text: string, search_text: string) {
  try {
    let alchemy_data = alchemy.InventoryAlchemy.load()
    let inventory = alchemy.Inventory.parse(inventory_text)
    recipes.value = inventory.recipes(alchemy_data, search_text)
    inventoryItems.value = inventory.items
  } catch (error) {
    console.error(error)
  }
}

watch(pastebox, (text, _) => {
  load_recipes(text, search.value)
})

watch(search, (search_text, _) => {
  load_recipes(pastebox.value, search_text)
})

function effectStyle(recipe: alchemy.Recipe): string {
  if (recipe.kind == alchemy.RecipeKind.Restore) {
    return "kind-restore"
  } else if (recipe.kind == alchemy.RecipeKind.Fortify) {
    return "kind-fortify"
  } else if (recipe.kind == alchemy.RecipeKind.Harm) {
    return "kind-harm"
  } else {
    return "kind-other"
  }
}
</script>

<template>
  <div class="alchemy">
    <section id="inventory-paste">
      <div>
        <textarea v-model="pastebox" cols="40" rows="25" placeholder="Paste your ingredient inventory here." />
      </div>
      <div v-if="showInventoryParse">
        <ul>
          <li v-for="item in inventoryItems">{{ item }}</li>
        </ul>
      </div>
    </section>

    <section id="recipes">
      <div v-if="inventoryItems.length > 0">
        <input type="text" placeholder="Search effect or ingredient" v-model="search"/>
      </div>

      <div>
        <table>
          <tr v-for="recipe in recipes" class="recipe">
            <td class="recipe-effect" :class="effectStyle(recipe)">{{ recipe.effect }}</td>
            <td class="recipe-value">{{ recipe.value }}</td>
            <td class="recipe-ingredient">
              <ul>
                <li v-for="ingredient in recipe.ingredients">{{ ingredient }}</li>
              </ul>
            </td>
          </tr>
        </table>
      </div>
    </section>
  </div>
</template>

<style scoped>
.alchemy {
  display: flex;
  flex-wrap: wrap;
}

.alchemy section {
  flex: auto;
}

.recipe td {
  vertical-align: top;
  padding-left: .25em;
  padding-bottom: .25em;
  border-bottom: 1px dashed lightgray;
  margin: 0;
}
#recipes > div {
  padding-top: .5em;
}

#recipes table {
  border-collapse: collapse;
  border: 1px solid lightgray;
}

#recipes table tr:last-child td {
  border-bottom: none
}

.recipe-effect {
  font-weight: bold;
}

.kind-restore {
  color: green;
}

.kind-harm {
  color: red;
}

.kind-fortify {
  color: blue;
}

.recipe-value {
  color: gold;
}
</style>