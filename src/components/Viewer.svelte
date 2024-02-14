<script lang="ts">
	export let collection_name: String;
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	let items: any = [];

	onMount(async () => {
		items = await invoke("fetch_items_command", { collection_name });
	});
</script>

{#each items as item}
	<p>{item.name}: {item.title}</p>
{/each}
