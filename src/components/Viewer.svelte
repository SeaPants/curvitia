<script lang="ts">
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
        <td><button on:click={() => fetchedItems.deleteItem(item._id.$oid)}
            >Delete</button></td>
      </tr>
    {/each}
  </tbody>
</table>
