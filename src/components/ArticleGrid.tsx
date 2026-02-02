import React from 'react';
import type { Article } from '../types';

interface ArticleGridProps {
  articles: Article[];
  viewedArticles: Set<string>;
  onArticleClick: (article: Article) => void;
  showImages: boolean;
  showExcerpts: boolean;
}

export function ArticleGrid({
  articles,
  viewedArticles,
  onArticleClick,
  showImages,
  showExcerpts,
}: ArticleGridProps) {
  if (articles.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">📭</div>
        <div className="empty-state-title">No Articles Yet</div>
        <div className="empty-state-text">
          Add a feed from the sidebar to get started
        </div>
      </div>
    );
  }

  return (
    <div className="articles-grid">
      {articles.map((article, index) => (
        <div
          key={index}
          className={`article-card ${
            viewedArticles.has(article.link) ? 'viewed' : ''
          }`}
          onClick={() => onArticleClick(article)}
        >
          {showImages && article.image_url && (
            <img
              src={article.image_url}
              alt={article.title}
              className="article-image"
            />
          )}
          <div className="article-content">
            <h3 className="article-title">{article.title}</h3>
            {showExcerpts && article.excerpt && (
              <p className="article-excerpt">{article.excerpt}</p>
            )}
          </div>
        </div>
      ))}
    </div>
  );
}
