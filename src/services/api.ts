import { invoke } from "@tauri-apps/api/core";
import type { Article, Feed, FeedData, AppSettings } from "../types";

export const api = {
  async addFeed(url: string): Promise<FeedData> {
    return await invoke("add_feed", { url });
  },

  async refreshFeed(url: string): Promise<FeedData> {
    return await invoke("refresh_feed", { url });
  },

  async getFeeds(): Promise<Feed[]> {
    return await invoke("get_feeds");
  },

  async markArticleViewed(url: string, title: string): Promise<void> {
    return await invoke("mark_article_viewed", { url, title });
  },

  async isArticleViewed(url: string): Promise<boolean> {
    return await invoke("is_article_viewed", { url });
  },

  async getViewedArticles(): Promise<string[]> {
    return await invoke("get_viewed_articles");
  },

  async getSettings(): Promise<AppSettings> {
    return await invoke("get_settings");
  },

  async updateSettings(newSettings: AppSettings): Promise<void> {
    return await invoke("update_settings", { newSettings });
  },
};
