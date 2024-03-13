<script lang="ts">
  import Home from './lib/Home.svelte';
  import Planner from './lib/Planner.svelte';
  import Recipes from './lib/Recipes.svelte';

  let selectedNavOption = 'home';

  function selectNavOption(option: string) {
    selectedNavOption = option;
  }
</script>

<main class="container">
  <!-- Main Content -->
  {#if selectedNavOption === 'home'}
    <Home />
  {:else if selectedNavOption === 'recipes'}
    <Recipes />
  {:else if selectedNavOption === 'planner'}
    <Planner />
  {/if}

  <!-- Bottom Navigation -->
  <div class="btm-nav z-50">
    <button
      class="{selectedNavOption === 'home' ? 'active' : ''} nav-button"
      on:click={() => selectNavOption('home')}
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
        <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
      </svg>      
    </button>
    <button
      class="{selectedNavOption === 'recipes' ? 'active' : ''} nav-button"
      on:click={() => selectNavOption('recipes')}
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
        <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
      </svg>       
    </button>
    <button
      class="{selectedNavOption === 'planner' ? 'active' : ''} nav-button"
      on:click={() => selectNavOption('planner')}
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
        <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
      </svg>      
    </button>
  </div>
</main>

<style>
  .nav-button:hover {
    @apply bg-base-300;
  }
</style>