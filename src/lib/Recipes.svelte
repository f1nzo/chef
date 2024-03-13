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

    async function delete_recipe(recipe_id: any) {
        console.log('Deleting recipe with id: ', recipe_id);
        invoke('delete_recipe', { recipeId: recipe_id });
        recipes = await invoke('load_recipes');
    }
</script>

<div class="flex flex-row h-12 w-full mt-4 mx-4 gap-5 justify-between">
    <!-- Search Bar -->
    <div class="flex flex-row gap-5 h-full w-full">
        <label class="input input-bordered flex items-center gap-2 5 w-full bg-base-100">
            <input type="text" class="grow" placeholder="Search" />
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70"><path fill-rule="evenodd" d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z" clip-rule="evenodd" /></svg>
        </label>
    </div>
    <!-- Filters -->
    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
    <ul class="menu menu-horizontal w-64 z-50 rounded-md bg-base-100">
        <li class="w-full h-full">
          <details class="w-full h-full">
            <summary class="flex justify-between items-center text-lg w-full h-full">
            Filters
            </summary>
            <ul class="bg-base-100 w-full rounded-lg">
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
    <button class="btn btn-square flex-shrink aspect-square hover-grow bg-base-100">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
          </svg>          
    </button>
</div>

<div class="grid grid-cols-4 gap-5 p-5 overflow-scroll content-center w-screen z-0 pb-[5.25rem]" style="grid-auto-rows: 1fr;">
    {#if recipes}
        {#each recipes as recipe}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="card bg-base-100 shadow-xl image-full hover-grow w-full aspect-square group">
                <figure><img src={recipe.image} alt={recipe.name} class="object-cover w-full h-full"/></figure>
                <div class="card-body text-center">
                    <h2 class="card-title text-white">{recipe.name}</h2>
                    <div class="card-actions absolute bottom-2 right-2">
                        <button class="btn btn-circle btn-ghost opacity-0 hover-view group-hover:opacity-100 shake" on:click={() => delete_recipe(recipe.id)}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                              </svg>                                                     
                        </button>
                    </div>
                    <div class="card-actions absolute bottom-2 left-2">
                        <button class="btn btn-circle btn-ghost opacity-0 hover-view group-hover:opacity-100 pulse" on:click={() => showModal(recipe)}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
                              </svg>                                                                                  
                        </button>
                    </div>
                </div>
            </div>
        {/each}
    {/if}
</div>

<dialog id="recipe_model" class="modal">
    <div class="modal-box">
        {#if currentRecipe}
            <h3 class="font-bold text-xl text-center bold">{currentRecipe.name}</h3>
            <div class="stats shadow w-full items-center">
                <div class="stat">
                  <div class="stat-figure text-secondary">
                    <span class="icon-[mdi--access-time]" style="width: 2.5rem; height: 2.5rem;"></span>
                  </div>
                  <div class="stat-title">Prep</div>
                  <div class="stat-value">{currentRecipe.prep_time}</div>
                  <div class="stat-desc">min</div>
                </div>
                
                <div class="stat">
                  <div class="stat-figure text-secondary">
                    <span class="icon-[mdi--campfire]" style="width: 2.5rem; height: 2.5rem;"></span>
                  </div>
                  <div class="stat-title">Cook</div>
                  <div class="stat-value">{currentRecipe.cook_time}</div>
                  <div class="stat-desc">min</div>
                </div>
                
                <div class="stat">
                  <div class="stat-figure text-secondary">
                    <span class="icon-[mdi--people]" style="width: 2.5rem; height: 2.5rem;"></span>                   
                  </div>
                  <div class="stat-title">Serves</div>
                  <div class="stat-value">{currentRecipe.serves}</div>
                  <div class="stat-desc">people</div>
                </div>
                
              </div>
        {/if}
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>

<style>
    .hover-grow:hover {
        transform: scale(1.1);
        transition: transform 0.25s;
    }

    @keyframes shake {
        0%, 100% {
            transform: rotate(0deg);
        }
        25% {
            transform: rotate(-10deg);
        }
        75% {
            transform: rotate(10deg);
        }
    }

    .shake:hover {
        animation: shake .25s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.1);
        }
    }

    .pulse:hover {
        animation: pulse 1s infinite;
    }
</style>