type Post = {
  slug: string;
  title: string;
  author: string;
  body: string;
};

export async function listPosts() {
  let response = await fetch("/api/posts");
  let posts: Post[] = await response.json();
  return posts;
}

export async function getPost(slug: string) {
  let response = await fetch(`/api/posts/${slug}`);
  let post: Post = await response.json();
  return post;
}
