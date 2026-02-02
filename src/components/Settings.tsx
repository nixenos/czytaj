import React from 'react';
import type { AppSettings, Theme } from '../types';

interface SettingsProps {
  settings: AppSettings;
  onUpdateSettings: (settings: AppSettings) => void;
  onBack: () => void;
}

export function Settings({ settings, onUpdateSettings, onBack }: SettingsProps) {
  const handleThemeChange = (theme: Theme) => {
    onUpdateSettings({ ...settings, theme });
  };

  const handleToggleImages = () => {
    onUpdateSettings({ ...settings, show_images: !settings.show_images });
  };

  const handleToggleExcerpts = () => {
    onUpdateSettings({ ...settings, show_excerpts: !settings.show_excerpts });
  };

  return (
    <div className="settings-page">
      <button className="back-button" onClick={onBack}>
        ← Back
      </button>

      <h2 className="settings-title">Settings</h2>

      <div className="settings-section">
        <label className="settings-label">Theme</label>
        <div className="theme-buttons">
          <button
            className={`theme-btn ${settings.theme === 'Light' ? 'active' : ''}`}
            onClick={() => handleThemeChange('Light')}
          >
            ☀️ Light
          </button>
          <button
            className={`theme-btn ${settings.theme === 'Dark' ? 'active' : ''}`}
            onClick={() => handleThemeChange('Dark')}
          >
            🌙 Dark
          </button>
        </div>
      </div>

      <div className="settings-section">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={settings.show_images}
            onChange={handleToggleImages}
            className="checkbox"
          />
          <span className="checkbox-text">Show article images</span>
        </label>

        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={settings.show_excerpts}
            onChange={handleToggleExcerpts}
            className="checkbox"
          />
          <span className="checkbox-text">Show article excerpts</span>
        </label>
      </div>
    </div>
  );
}
