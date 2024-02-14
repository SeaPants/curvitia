<script lang="ts">
  import { push } from 'svelte-spa-router';
</script>

<main class="container">
  <h1>Curvitia</h1>
  <img src="/logo.svg" class="logo curvitia" alt="Curvitia Logo" />

  <div class="row">
    <button on:click={() => push('/')}> Home </button>
    <button on:click={() => push('/personal-data')}> Personal Data </button>
    <button on:click={() => push('/experiences')}> Experiences </button>
    <button on:click={() => push('/works')}> Works </button>
  </div>
</main>
