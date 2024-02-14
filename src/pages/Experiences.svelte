<script lang="ts">
  import useCollectionController from '../lib/useCollectionController';
  import Editor from '../components/Editor.svelte';
  import Viewer from '../components/Viewer.svelte';

  const collectionName = 'experiences';
  const myCollectionController = useCollectionController(collectionName);
</script>

<main class="container">
  <Editor fetchedItems={myCollectionController} />
  <Viewer fetchedItems={myCollectionController} />
</main>
