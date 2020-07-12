<script>
  import { user } from "./store.js";
  import { Link } from "yrv";

  let isActive = false;
  function toggleMenu() {
    isActive = !isActive;
  }
</script>

<style>
  :global([aria-current]) {
    font-weight: bold;
  }
</style>

<nav role="navigation" aria-label="main navigation">
  <strong>cstatus</strong>
</nav>
