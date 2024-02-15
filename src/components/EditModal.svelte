<script lang="ts">
	import type { Mode } from '../lib/types';

	export let contentComponent: any;
	export let fetchedItems: any;
	export let mode: Mode = 'create';
	export let itemId: string | null = null;

	let showModal = false;

	let buttonLabel = {
		create: 'Create',
		update: 'Edit',
		delete: 'Delete',
	}[mode];

	function toggleModal() {
		showModal = !showModal;
	}
	function handleClose() {
		showModal = false;
	}
</script>

<button on:click={toggleModal}>{buttonLabel}</button>

{#if showModal}
	<div class="modal-background" on:click={toggleModal}>
		<div class="modal" on:click|stopPropagation>
			<svelte:component
				this={contentComponent}
				{fetchedItems}
				{mode}
				{itemId}
				on:complete={handleClose} />
			<br />
			<button on:click={toggleModal}>Close</button>
		</div>
	</div>
{/if}

<style>
	.modal-background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
	}
	.modal {
		background-color: white;
		padding: 20px;
		border-radius: 5px;
	}
</style>
