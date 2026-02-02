export interface Article {
  title: string;
  link: string;
  excerpt: string | null;
  image_url: string | null;
}

export interface Feed {
  url: string;
  title: string;
}

export interface FeedData {
  title: string;
  articles: Article[];
}

export type Theme = "Light" | "Dark";

export interface AppSettings {
  theme: Theme;
  show_images: boolean;
  show_excerpts: boolean;
}
