<script>
    let recipes = [
        {
            name: 'Ramen',
            image: 'https://media.istockphoto.com/id/511793034/photo/japanese-ramen-soup-with-chicken-egg-chives-and-sprout.jpg?b=1&s=612x612&w=0&k=20&c=SpW8Op5RozB2pGVn6EJcZKd_xwND7zL923RJkvVx5IE='
        },
        {
            name: 'Sushi',
            image: 'https://images.pexels.com/photos/12955612/pexels-photo-12955612.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burrito',
            image: 'https://images.pexels.com/photos/2955819/pexels-photo-2955819.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Pizza',
            image: 'https://www.southernliving.com/thmb/j_6gABRIAMegN6RFHxOgbUqBxjA=/750x0/filters:no_upscale():max_bytes(150000):strip_icc():format(webp)/2652401_QFSSL_SupremePizza_00072-d910a935ba7d448e8c7545a963ed7101.jpg'
        },
        {
            name: 'Pasta',
            image: 'https://images.pexels.com/photos/3209101/pexels-photo-3209101.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burger',
            image: 'https://static.independent.co.uk/s3fs-public/thumbnails/image/2019/04/26/09/mcdonalds-bigvegants.jpg'
        },
        {
            name: 'Ramen',
            image: 'https://media.istockphoto.com/id/511793034/photo/japanese-ramen-soup-with-chicken-egg-chives-and-sprout.jpg?b=1&s=612x612&w=0&k=20&c=SpW8Op5RozB2pGVn6EJcZKd_xwND7zL923RJkvVx5IE='
        },
        {
            name: 'Sushi',
            image: 'https://images.pexels.com/photos/12955612/pexels-photo-12955612.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burrito',
            image: 'https://images.pexels.com/photos/2955819/pexels-photo-2955819.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Pizza',
            image: 'https://www.southernliving.com/thmb/j_6gABRIAMegN6RFHxOgbUqBxjA=/750x0/filters:no_upscale():max_bytes(150000):strip_icc():format(webp)/2652401_QFSSL_SupremePizza_00072-d910a935ba7d448e8c7545a963ed7101.jpg'
        },
        {
            name: 'Pasta',
            image: 'https://images.pexels.com/photos/3209101/pexels-photo-3209101.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burger',
            image: 'https://static.independent.co.uk/s3fs-public/thumbnails/image/2019/04/26/09/mcdonalds-bigvegants.jpg'
        },
        {
            name: 'Ramen',
            image: 'https://media.istockphoto.com/id/511793034/photo/japanese-ramen-soup-with-chicken-egg-chives-and-sprout.jpg?b=1&s=612x612&w=0&k=20&c=SpW8Op5RozB2pGVn6EJcZKd_xwND7zL923RJkvVx5IE='
        },
        {
            name: 'Sushi',
            image: 'https://images.pexels.com/photos/12955612/pexels-photo-12955612.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burrito',
            image: 'https://images.pexels.com/photos/2955819/pexels-photo-2955819.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Pizza',
            image: 'https://www.southernliving.com/thmb/j_6gABRIAMegN6RFHxOgbUqBxjA=/750x0/filters:no_upscale():max_bytes(150000):strip_icc():format(webp)/2652401_QFSSL_SupremePizza_00072-d910a935ba7d448e8c7545a963ed7101.jpg'
        },
        {
            name: 'Pasta',
            image: 'https://images.pexels.com/photos/3209101/pexels-photo-3209101.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1'
        },
        {
            name: 'Burger',
            image: 'https://static.independent.co.uk/s3fs-public/thumbnails/image/2019/04/26/09/mcdonalds-bigvegants.jpg'
        }
    ]

    let currentRecipe = null; // Reactive variable for the current recipe

    function showModal(recipe) {
        currentRecipe = recipe; // Update the current recipe
        const modal = document.getElementById('recipe_model');
        modal.showModal(); // Using DaisyUI's showModal function
    }

</script>

<div class="flex flex-row mt-5 mx-5 gap-5">
    <label class="input input-bordered flex items-center gap-2 5 w-2/3">
        <input type="text" class="grow" placeholder="Search" />
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70"><path fill-rule="evenodd" d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z" clip-rule="evenodd" /></svg>
    </label>
</div>

<div class="grid grid-cols-4 gap-5 p-5 overflow-scroll content-center w-screen" style="grid-auto-rows: 1fr;">
    {#each recipes as recipe}
        <div class="card bg-base-100 shadow-xl image-full hover-grow" on:click="{() => showModal(recipe)}">
            <figure><img src={recipe.image} alt={recipe.name} class="object-cover w-full h-full"/></figure>
            <div class="card-body text-center">
                <h2 class="card-title text-white">{recipe.name}</h2>
                <div class="card-actions absolute bottom-2 right-2">
                </div>
            </div>
        </div>
    {/each}
</div>

<dialog id="recipe_model" class="modal">
    <div class="modal-box">
        {#if currentRecipe}
            <h3 class="font-bold text-lg">{currentRecipe.name}</h3>
            <img src="{currentRecipe.image}" alt="{currentRecipe.name}" class="py-5"/>
        {/if}
    </div>
    <form method="dialog" class="modal-backdrop">
        <button type="button" on:click="{() => document.getElementById('recipe_model').close()}">Close</button>
    </form>
</dialog>

<style>
    .hover-grow:hover {
        transform: scale(1.05);
        transition: transform 0.3s;
    }
</style>