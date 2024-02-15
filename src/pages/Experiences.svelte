<script lang="ts">
  import useCollectionController from '../lib/useCollectionController';
  import ExperienceForm from '../components/ExperienceForm.svelte';
  import Viewer from '../components/Viewer.svelte';
  import EditModal from '../components/EditModal.svelte';

  const collectionName = 'experiences';
  const myCollectionController = useCollectionController(collectionName);

  let myEditor = ExperienceForm;
</script>

<main class="container">
  <EditModal
    contentComponent={myEditor}
    fetchedItems={myCollectionController}
    mode="create" />

  <Viewer contentComponent={myEditor} fetchedItems={myCollectionController} />
</main>
