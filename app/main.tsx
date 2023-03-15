import { NavBar } from "@/components/NavBar";
import { Route, Router, Routes } from "@solidjs/router";
import { lazy } from "solid-js";
import { render } from "solid-js/web";
import "@/style.css";

const IndexPage = lazy(() => import("@/pages/index"));
const AboutPage = lazy(() => import("@/pages/about"));
const PostPage = lazy(() => import("@/pages/post"));

function Main() {
  return (
    <div class="justify-center sm:px-8">
      <div class="flex w-full max-w-7xl lg:px-8">
        <div class="container">
          <NavBar />
          <Routes>
            <Route path="/" component={IndexPage} />
            <Route path="/about" component={AboutPage} />
            <Route path="/posts/:slug" component={PostPage} />
          </Routes>
        </div>
      </div>
    </div>
  );
}

const el = document.getElementById("app");
if (el === null) {
  throw new Error("unable to find app element");
}

render(
  () => (
    <Router>
      <Main />
    </Router>
  ),
  el
);
