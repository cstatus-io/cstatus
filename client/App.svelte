<script>
  import Nav from "./Nav.svelte";
  import Footer from "./Footer.svelte";
  import { Router, Route } from "yrv";
  import Home from "./pages/Home.svelte";
  import About from "./pages/About.svelte";
  import Common from "./pages/Common.svelte";
  import Catcher from "./pages/Catcher.svelte";
  import { user } from "./store.js";
</script>

<main>
  <Nav />
  <Router>
    <Route exact path="/">
      <Home />
    </Route>
    <Route path="/about">
      <About />
    </Route>
    <Route path="/app/common" let:router>
      <Common />
    </Route>
  </Router>
  <Footer />
</main>
