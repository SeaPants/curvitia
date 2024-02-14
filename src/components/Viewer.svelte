<script lang="ts">
	export let collection_name: String;
	import { invoke } from "@tauri-apps/api/tauri";
	import { writable } from "svelte/store";
	let items = writable([]);

	async function fetchItems() {
		const fetchedItems = await invoke("fetch_items_command", { collection_name });
		items.set(fetchedItems);
	}

	fetchItems();

	let name = "";
	let description = "";

	async function createItem() {
		const item = { name, description };
		await invoke("create_item_command", { collection_name, item });
		fetchItems();
	}

	async function deleteItem(item_id) {
		await invoke("delete_item_command", { collection_name, item_id });
		fetchItems();
	}
</script>

<form on:submit|preventDefault={createItem}>
	<input type="text" bind:value={name} placeholder="Name" />
	<input type="text" bind:value={description} placeholder="Description" />
	<button type="submit">Create Item</button>
</form>

{#each $items as item}
	<main class="container">
		<p>{item.name}: {item.description} {item._id.$oid}</p>
		<button on:click={() => deleteItem(item._id.$oid)}>Delete</button>
	</main>
{/each}
