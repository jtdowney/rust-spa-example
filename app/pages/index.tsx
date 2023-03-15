import { A } from "@solidjs/router";
import { createResource, For, Show } from "solid-js";
import { listPosts } from "~/api";

export default function Index() {
  const [posts] = createResource(listPosts);

  return (
    <main>
      <Show when={!posts.loading} fallback={<p>Loading...</p>}>
        <ul>
          <For each={posts()}>
            {(post) => (
              <li>
                <A href={`/posts/${post.slug}`}>{post.title}</A>
              </li>
            )}
          </For>
        </ul>
      </Show>
    </main>
  );
}
