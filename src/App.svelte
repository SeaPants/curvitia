<script>
  import Router from "svelte-spa-router";
  import Header from "./components/Header.svelte";
  import Index from "./pages/Home.svelte";
  import Experiences from "./pages/Experiences.svelte";

  const routes = {
    "/": Index,
    "/experiences": Experiences,
    "*": Index,
  };
</script>

<main>
  <Header />
  <Router {routes}></Router>
</main>
