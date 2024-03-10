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

<div class="flex flex-row mt-5 mx-5 gap-5">
    <label class="input input-bordered flex items-center gap-2 5 w-2/3">
        <input type="text" class="grow" placeholder="Search" />
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70"><path fill-rule="evenodd" d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z" clip-rule="evenodd" /></svg>
    </label>
</div>

<div class="grid grid-cols-4 gap-5 p-5 overflow-scroll content-center w-screen" style="grid-auto-rows: 1fr;">
    {#if recipes}
        {#each recipes as recipe}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="card bg-base-100 shadow-xl image-full hover-grow" on:click="{() => showModal(recipe)}">
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