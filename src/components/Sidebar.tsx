import React, { useState } from 'react';
import type { Feed } from '../types';

interface SidebarProps {
  feeds: Feed[];
  isCollapsed: boolean;
  onAddFeed: (url: string) => void;
  onRefreshFeed: (url: string) => void;
  onOpenSettings: () => void;
  loading: boolean;
}

export function Sidebar({
  feeds,
  isCollapsed,
  onAddFeed,
  onRefreshFeed,
  onOpenSettings,
  loading,
}: SidebarProps) {
  const [feedInput, setFeedInput] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (feedInput.trim()) {
      onAddFeed(feedInput.trim());
      setFeedInput('');
    }
  };

  return (
    <div className={`sidebar ${isCollapsed ? 'collapsed' : ''}`}>
      <div className="sidebar-header">
        <h1>📰 Czytaj</h1>
        <form onSubmit={handleSubmit} className="feed-input-container">
          <input
            type="url"
            placeholder="Enter RSS/Atom feed URL"
            value={feedInput}
            onChange={(e) => setFeedInput(e.target.value)}
            className="feed-input"
            disabled={loading}
          />
          <button
            type="submit"
            className="btn btn-primary"
            disabled={loading || !feedInput.trim()}
          >
            {loading ? 'Loading...' : 'Add Feed'}
          </button>
        </form>
      </div>

      <div className="feeds-list">
        {feeds.length === 0 ? (
          <div className="empty-state">
            <p className="empty-state-text">No feeds yet. Add one above!</p>
          </div>
        ) : (
          feeds.map((feed, index) => (
            <div
              key={index}
              className="feed-item"
              onClick={() => onRefreshFeed(feed.url)}
            >
              <div className="feed-item-title">{feed.title}</div>
              <div className="feed-item-url">{feed.url}</div>
            </div>
          ))
        )}
      </div>

      <div className="sidebar-footer">
        <button className="btn btn-secondary" onClick={onOpenSettings}>
          ⚙️ Settings
        </button>
      </div>
    </div>
  );
}
