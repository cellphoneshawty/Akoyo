<script lang="ts">
  import Icon from '../components/Icon.svelte';
  import Logo from '../components/Logo.svelte';
  import type { User } from '../lib/api';
  import { t } from '../lib/i18n.svelte';
  import type { MessageKey } from '../lib/locales/en';
  import { href, router, type Route } from '../lib/router.svelte';
  import LikedPage from '../pages/LikedPage.svelte';
  import SearchPage from '../pages/SearchPage.svelte';
  import SettingsPage from '../pages/SettingsPage.svelte';

  let { user }: { user: User } = $props();

  const nav: { route: Route; icon: 'search' | 'heart' | 'settings'; label: MessageKey }[] = [
    { route: 'search', icon: 'search', label: 'nav.search' },
    { route: 'liked', icon: 'heart', label: 'nav.liked' },
    { route: 'settings', icon: 'settings', label: 'nav.settings' },
  ];
</script>

<div class="shell">
  <nav class="sidebar">
    <a class="brand" href={href('search')}><Logo withName /></a>
    <ul>
      {#each nav as item (item.route)}
        <li>
          <a
            href={href(item.route)}
            class:active={router.route === item.route}
            aria-current={router.route === item.route ? 'page' : undefined}
          >
            <Icon name={item.icon} />
            <span>{t(item.label)}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <main class="content">
    {#if router.route === 'search'}
      <SearchPage />
    {:else if router.route === 'liked'}
      <LikedPage />
    {:else}
      <SettingsPage {user} />
    {/if}
  </main>

  <footer class="player">
    <div class="now-playing">
      <div class="cover"><Icon name="music" size={20} /></div>
      <span class="idle">{t('player.idle')}</span>
    </div>
    <div class="controls">
      <button aria-label="Previous" disabled><Icon name="prev" size={20} /></button>
      <button class="play" aria-label="Play" disabled><Icon name="play" size={22} /></button>
      <button aria-label="Next" disabled><Icon name="next" size={20} /></button>
    </div>
    <div class="spacer"></div>
  </footer>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 240px 1fr;
    grid-template-rows: 1fr auto;
    grid-template-areas:
      'sidebar content'
      'player player';
    height: 100%;
  }

  .sidebar {
    grid-area: sidebar;
    display: flex;
    flex-direction: column;
    gap: 24px;
    padding: 22px 14px;
    border-right: 1px solid var(--border);
    background: var(--bg-elevated);
  }

  .brand {
    padding: 0 10px;
    color: var(--text);
  }

  .brand:hover {
    text-decoration: none;
  }

  ul {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  ul a {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-weight: 600;
    transition: background 0.15s, color 0.15s;
  }

  ul a:hover {
    background: var(--surface);
    color: var(--text);
    text-decoration: none;
  }

  ul a.active {
    background: var(--surface-hover);
    color: var(--text);
  }

  .content {
    grid-area: content;
    overflow-y: auto;
  }

  .player {
    grid-area: player;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 16px;
    height: 76px;
    padding: 0 20px;
    border-top: 1px solid var(--border);
    background: var(--bg-elevated);
  }

  .now-playing {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }

  .cover {
    display: grid;
    flex-shrink: 0;
    place-items: center;
    width: 46px;
    height: 46px;
    border-radius: 8px;
    background: var(--surface);
    color: var(--text-faint);
  }

  .idle {
    overflow: hidden;
    color: var(--text-faint);
    font-size: 14px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .controls button {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border: 0;
    border-radius: 50%;
    background: none;
    color: var(--text-muted);
  }

  .controls button:disabled {
    opacity: 0.45;
  }

  .controls .play {
    width: 42px;
    height: 42px;
    background: var(--text);
    color: var(--bg);
  }

  /* Phones: navigation moves to a bottom tab bar under the player. */
  @media (max-width: 720px) {
    .shell {
      grid-template-columns: 1fr;
      grid-template-rows: 1fr auto auto;
      grid-template-areas:
        'content'
        'player'
        'sidebar';
    }

    .sidebar {
      padding: 6px 8px calc(6px + env(safe-area-inset-bottom));
      border-top: 1px solid var(--border);
      border-right: 0;
    }

    .brand {
      display: none;
    }

    ul {
      flex-direction: row;
      justify-content: space-around;
    }

    ul a {
      flex-direction: column;
      gap: 2px;
      padding: 6px 12px;
      font-size: 11px;
    }

    ul a.active {
      background: none;
      color: var(--accent-strong);
    }

    .player {
      grid-template-columns: 1fr auto;
      height: 64px;
      padding: 0 16px;
    }

    .controls button:not(.play),
    .spacer {
      display: none;
    }
  }
</style>
