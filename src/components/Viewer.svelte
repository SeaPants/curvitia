<script lang="ts">
  import EditModal from './EditModal.svelte';

  export let contentComponent: any;
  export let fetchedItems: any;

  function objectEntries(obj: any): [string, any][] {
    return Object.entries(obj);
  }
</script>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Fields</th>
      <th>Edit / Delete</th>
    </tr>
  </thead>
  <tbody>
    {#each $fetchedItems as item}
      <tr>
        <td>{item._id.$oid}</td>
        <td>
          <ul>
            {#each objectEntries(item) as [key, value]}
              {#if key !== '_id'}
                <li>{key}: {JSON.stringify(value)}</li>
              {/if}
            {/each}
          </ul>
        </td>
        <td>
          <EditModal
            {contentComponent}
            {fetchedItems}
            mode="update"
            itemId={item._id.$oid} />
          <EditModal
            {contentComponent}
            {fetchedItems}
            mode="delete"
            itemId={item._id.$oid} />
        </td>
      </tr>
    {/each}
  </tbody>
</table>
