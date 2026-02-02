import React, { useState, useEffect } from 'react';
import { Sidebar } from './components/Sidebar';
import { ArticleGrid } from './components/ArticleGrid';
import { ArticleDetail } from './components/ArticleDetail';
import { Settings } from './components/Settings';
import { api } from './services/api';
import type { Article, Feed, AppSettings } from './types';
import './styles/App.css';

type View = 'articles' | 'detail' | 'settings';

function App() {
  const [feeds, setFeeds] = useState<Feed[]>([]);
  const [articles, setArticles] = useState<Article[]>([]);
  const [currentArticle, setCurrentArticle] = useState<Article | null>(null);
  const [viewedArticles, setViewedArticles] = useState<Set<string>>(new Set());
  const [settings, setSettings] = useState<AppSettings>({
    theme: 'Light',
    show_images: true,
    show_excerpts: true,
  });
  const [view, setView] = useState<View>('articles');
  const [loading, setLoading] = useState(false);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  // Load initial data
  useEffect(() => {
    loadInitialData();
  }, []);

  // Apply theme
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', settings.theme);
  }, [settings.theme]);

  const loadInitialData = async () => {
    try {
      const [loadedSettings, loadedFeeds, loadedViewedArticles] = await Promise.all([
        api.getSettings(),
        api.getFeeds(),
        api.getViewedArticles(),
      ]);
      setSettings(loadedSettings);
      setFeeds(loadedFeeds);
      setViewedArticles(new Set(loadedViewedArticles));
    } catch (error) {
      console.error('Failed to load initial data:', error);
    }
  };

  const handleAddFeed = async (url: string) => {
    setLoading(true);
    try {
      const feedData = await api.addFeed(url);
      setFeeds([...feeds, { url, title: feedData.title }]);
      setArticles([...articles, ...feedData.articles]);
    } catch (error) {
      console.error('Failed to add feed:', error);
      alert('Failed to add feed. Please check the URL and try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleRefreshFeed = async (url: string) => {
    setLoading(true);
    try {
      const feedData = await api.refreshFeed(url);
      // Remove old articles from this feed and add new ones
      const otherArticles = articles.filter(
        (a) => !feeds.find((f) => f.url === url)
      );
      setArticles([...otherArticles, ...feedData.articles]);
    } catch (error) {
      console.error('Failed to refresh feed:', error);
      alert('Failed to refresh feed.');
    } finally {
      setLoading(false);
    }
  };

  const handleArticleClick = async (article: Article) => {
    setCurrentArticle(article);
    setView('detail');
    
    // Mark as viewed
    try {
      await api.markArticleViewed(article.link, article.title);
      setViewedArticles(new Set([...viewedArticles, article.link]));
    } catch (error) {
      console.error('Failed to mark article as viewed:', error);
    }
  };

  const handleBackToList = () => {
    setView('articles');
    setCurrentArticle(null);
  };

  const handleOpenSettings = () => {
    setView('settings');
  };

  const handleUpdateSettings = async (newSettings: AppSettings) => {
    try {
      await api.updateSettings(newSettings);
      setSettings(newSettings);
    } catch (error) {
      console.error('Failed to update settings:', error);
    }
  };

  const handleBackFromSettings = () => {
    setView('articles');
  };

  return (
    <div className="app-container">
      <Sidebar
        feeds={feeds}
        isCollapsed={sidebarCollapsed}
        onAddFeed={handleAddFeed}
        onRefreshFeed={handleRefreshFeed}
        onOpenSettings={handleOpenSettings}
        loading={loading}
      />

      <div className="main-content">
        <div className="header">
          <button
            className="toggle-sidebar-btn"
            onClick={() => setSidebarCollapsed(!sidebarCollapsed)}
          >
            ☰
          </button>
          <h2 className="header-title">
            {view === 'articles' && 'Articles'}
            {view === 'detail' && 'Article Detail'}
            {view === 'settings' && 'Settings'}
          </h2>
        </div>

        <div className="content-area">
          {view === 'articles' && (
            <ArticleGrid
              articles={articles}
              viewedArticles={viewedArticles}
              onArticleClick={handleArticleClick}
              showImages={settings.show_images}
              showExcerpts={settings.show_excerpts}
            />
          )}

          {view === 'detail' && currentArticle && (
            <ArticleDetail
              article={currentArticle}
              onBack={handleBackToList}
              showImages={settings.show_images}
            />
          )}

          {view === 'settings' && (
            <Settings
              settings={settings}
              onUpdateSettings={handleUpdateSettings}
              onBack={handleBackFromSettings}
            />
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
