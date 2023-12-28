<script setup lang="ts">
import { ref, watch } from 'vue';
import skyrim_alchemy_wasm_init, * as alchemy from 'skyrim_alchemy_wasm'

skyrim_alchemy_wasm_init().then(() => {
  console.log("alchemy initialized")
})

let pastebox = ref<string>("")
let recipes = ref<alchemy.Recipe[]>([])

watch(pastebox, (text, _) => {
  try {
    let inventory = alchemy.Inventory.parse(text)
    recipes.value = inventory.recipes()
  } catch (error) {
    console.error(error)
  }
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
      <textarea v-model="pastebox" cols="40" rows="25" placeholder="Paste your ingredient inventory here."/>
    </section>

    <section id="recipes">
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