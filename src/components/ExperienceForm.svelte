<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Mode } from '../lib/types';

  export let fetchedItems: any;
  export let mode: Mode = 'create';
  export let itemId: string | null = null;
  const dispatch = createEventDispatcher();

  let buttonLabel = {
    create: 'Create',
    update: 'Edit',
    delete: 'Delete',
  }[mode];

  interface ExperienceLocaleInfo {
    locale: 'en' | 'ja';
    name: string;
    description: string;
    specificAchievements: string[];
  }

  interface Experience {
    mainLocaleInfo: ExperienceLocaleInfo;
    altLocaleInfo?: ExperienceLocaleInfo;
    startDate: string;
    endDate: string;
  }

  function handleSubmit() {
    switch (mode) {
      case 'create':
        fetchedItems.createItem({ name: name, description: description });
        console.log('here is createItem');
        break;
      case 'update':
        // fetchedItems.updateItem({ name: name, description: description });
        console.log('here is updateItem');
        break;
      case 'delete':
        fetchedItems.deleteItem(itemId);
        console.log('here is deleteItem');
        break;
    }

    dispatch('complete', { message: 'Form completed' });
  }

  let name = '';
  let description = '';
</script>

<form on:submit|preventDefault={handleSubmit}>
  <input type="text" bind:value={name} placeholder="Name" />
  <input type="text" bind:value={description} placeholder="Description" />
  <button type="submit">{buttonLabel}</button>
</form>
