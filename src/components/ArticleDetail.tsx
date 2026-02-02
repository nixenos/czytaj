import React from 'react';
import { open } from '@tauri-apps/plugin-shell';
import type { Article } from '../types';

interface ArticleDetailProps {
  article: Article;
  onBack: () => void;
  showImages: boolean;
}

export function ArticleDetail({ article, onBack, showImages }: ArticleDetailProps) {
  const handleOpenLink = async () => {
    try {
      await open(article.link);
    } catch (error) {
      console.error('Failed to open link:', error);
    }
  };

  return (
    <div className="article-detail">
      <button className="back-button" onClick={onBack}>
        ← Back to articles
      </button>

      {showImages && article.image_url && (
        <img
          src={article.image_url}
          alt={article.title}
          className="article-detail-image"
        />
      )}

      <h1 className="article-detail-title">{article.title}</h1>

      {article.excerpt && (
        <p className="article-detail-excerpt">{article.excerpt}</p>
      )}

      <a
        className="article-detail-link"
        onClick={handleOpenLink}
        style={{ cursor: 'pointer' }}
      >
        🔗 Read full article
      </a>
    </div>
  );
}
