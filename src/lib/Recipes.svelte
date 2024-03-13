<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';

    let recipes: any = [];

    onMount(async () => {
        recipes = await invoke('load_recipes');
    });

    let currentRecipe: any = null; // Reactive variable for the current recipe

    function showModal(recipe: any) {
        currentRecipe = recipe; // Update the current recipe
        const modal = document.getElementById('recipe_model') as HTMLDialogElement;
        if (modal) {
            modal.showModal();
        }
    }
</script>

<div class="flex flex-row h-12 w-full mt-4 mx-4 gap-5 justify-between">
    <!-- Search Bar -->
    <div class="flex flex-row gap-5 h-full w-full">
        <label class="input input-bordered flex items-center gap-2 5 w-full">
            <input type="text" class="grow" placeholder="Search" />
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70"><path fill-rule="evenodd" d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z" clip-rule="evenodd" /></svg>
        </label>
    </div>
    <!-- Filters -->
    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
    <ul class="menu menu-horizontal w-64 z-50 rounded-md bg-base-200">
        <li class="w-full h-full">
          <details class="w-full h-full">
            <summary class="flex justify-between items-center text-lg bg-base-200 w-full h-full">
            Filters
            </summary>
            <ul class="bg-base-200 w-full">
              <li class="flex flex-col w-full h-8">
                <div class="form-control w-full h-full flex">
                    <label class="label cursor-pointer w-full h-full flex justify-between">
                        <span class="label-text">Option 1</span> 
                        <input type="checkbox" checked="checked" class="checkbox checkbox-xs" />
                    </label>
                </div>
              </li>
              <li class="flex flex-col w-full h-8">
                <div class="form-control w-full h-full flex">
                    <label class="label cursor-pointer w-full h-full flex justify-between">
                        <span class="label-text">Option 2</span> 
                        <input type="checkbox" checked="checked" class="checkbox checkbox-xs" />
                    </label>
                </div>
              </li>
              <li class="flex flex-col w-full h-8">
                <div class="form-control w-full h-full flex">
                    <label class="label cursor-pointer w-full h-full flex justify-between">
                        <span class="label-text">Option 3</span> 
                        <input type="checkbox" checked="checked" class="checkbox checkbox-xs" />
                    </label>
                </div>
              </li>
            </ul>
          </details>
        </li>
      </ul>
    <!-- Add Recipe Button -->
    <button class="btn btn-square flex-shrink aspect-square">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
          </svg>          
    </button>
</div>

<div class="grid grid-cols-4 gap-5 p-5 overflow-scroll content-center w-screen z-0" style="grid-auto-rows: 1fr;">
    {#if recipes}
        {#each recipes as recipe}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="card bg-base-100 shadow-xl image-full hover-grow w-full aspect-square" on:click="{() => showModal(recipe)}">
                <figure><img src={recipe.image} alt={recipe.name} class="object-cover w-full h-full"/></figure>
                <div class="card-body text-center">
                    <h2 class="card-title text-white">{recipe.name}</h2>
                    <div class="card-actions absolute bottom-2 right-2">
                    </div>
                </div>
            </div>
        {/each}
    {/if}
</div>

<dialog id="recipe_model" class="modal">
    <div class="modal-box">
        {#if currentRecipe}
            <h3 class="font-bold text-lg">{currentRecipe.name}</h3>
            <img src="{currentRecipe.image}" alt="{currentRecipe.name}" class="py-5"/>
        {/if}
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>

<style>
    .hover-grow:hover {
        transform: scale(1.05);
        transition: transform 0.3s;
    }
</style>