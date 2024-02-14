<script lang="ts">
  export let fetchedItems: any;

  let name = '';
  let description = '';
</script>

<form
    on:submit|preventDefault={() =>
    fetchedItems.createItem({ name: name, description: description })}>
  <input type="text" bind:value={name} placeholder="Name" />
  <input type="text" bind:value={description} placeholder="Description" />
  <button type="submit">Create Item</button>
</form>
