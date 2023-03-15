import { useParams } from "@solidjs/router";
import { createResource, Show } from "solid-js";
import { getPost } from "~/api";

type PostParams = {
  slug: string;
};

export default function Post() {
  const params = useParams<PostParams>();
  const [post] = createResource(params.slug, getPost);
  return (
    <Show when={post()} fallback={<p>Loading...</p>} keyed>
      {(post) => (
        <article class="prose md:prose-lg lg:prose-xl prose-stone">
          <h1 class="text-3xl">{post.title}</h1>
          {post.body.split("\n\n").map((paragraph) => (
            <p>
              {paragraph
                .split("\n")
                .reduce((acc, line) => [acc, <br />, line].join(""))}
            </p>
          ))}
        </article>
      )}
    </Show>
  );
}
