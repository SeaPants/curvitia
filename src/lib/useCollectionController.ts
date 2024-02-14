import { invoke } from "@tauri-apps/api/tauri";
import { writable } from "svelte/store";

const fetchedItems = writable([]);

function useCollectionController(collectionName: string) {
	const fetchItems = async () => {
		fetchedItems.set(await invoke("fetch_items_command", { collection_name: collectionName }));
	};

	const createItem = async (item: any) => {
		await invoke("create_item_command", { collection_name: collectionName, item: item });
		await fetchItems();
	};

	const deleteItem = async (itemId: string) => {
		await invoke("delete_item_command", { collection_name: collectionName, item_id: itemId });
		await fetchItems();
	};

	fetchItems();

	return {
		subscribe: fetchedItems.subscribe,
		fetchItems,
		createItem,
		deleteItem,
	};
}

export default useCollectionController;
