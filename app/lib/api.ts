type Post = {
  slug: string;
  title: string;
  author: string;
  body: string;
};

export async function listPosts() {
  const response = await fetch("/api/posts");
  const posts: Post[] = await response.json();
  return posts;
}

export async function getPost(slug: string) {
  const response = await fetch(`/api/posts/${slug}`);
  const post: Post = await response.json();
  return post;
}
