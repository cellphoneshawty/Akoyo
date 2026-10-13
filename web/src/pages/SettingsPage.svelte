<script lang="ts">
  import LanguagePicker from '../components/LanguagePicker.svelte';
  import PlatformBadge from '../components/PlatformBadge.svelte';
  import { api, type User } from '../lib/api';
  import { t } from '../lib/i18n.svelte';
  import { platforms } from '../lib/platforms';
  import { session, signOut } from '../lib/session.svelte';

  let { user }: { user: User } = $props();

  async function changeLanguage(language: string) {
    try {
      session.user = await api.updateMe({ language });
    } catch {
      // the interface already switched; saving the preference can be retried later
    }
  }
</script>

<div class="page">
  <h1 class="page-title">{t('settings.title')}</h1>

  <section>
    <h2>{t('settings.account')}</h2>
    <div class="card row">
      <div class="avatar">{user.username.slice(0, 1).toUpperCase()}</div>
      <div class="grow">
        <div class="username">{user.username}</div>
        {#if user.is_admin}<span class="tag">{t('settings.admin')}</span>{/if}
      </div>
      <button class="btn" onclick={signOut}>{t('settings.signOut')}</button>
    </div>
  </section>

  <section>
    <h2>{t('settings.language')}</h2>
    <LanguagePicker onchange={changeLanguage} />
  </section>

  <section>
    <h2>{t('settings.services')}</h2>
    <p class="muted">{t('settings.servicesText')}</p>
    <div class="services">
      {#each platforms as platform (platform.id)}
        <div class="card row">
          <div class="grow">
            <PlatformBadge {platform} />
            {#if platform.experimental}<span class="tag">{t('settings.experimental')}</span>{/if}
          </div>
          {#if platform.available}
            <button class="btn btn-primary">{t('settings.connect')}</button>
          {:else}
            <span class="tag">{t('settings.comingSoon')}</span>
          {/if}
        </div>
      {/each}
    </div>
  </section>

  <section>
    <h2>{t('settings.offline')}</h2>
    <p class="muted">{t('settings.offlineText')}</p>
  </section>

  <section>
    <h2>{t('settings.about')}</h2>
    <p class="muted">Akoyo · {t('settings.version', { version: session.version })}</p>
  </section>
</div>

<style>
  .page {
    max-width: 760px;
  }

  section {
    margin-top: 36px;
  }

  h2 {
    margin-bottom: 12px;
    font-size: 18px;
  }

  .muted {
    margin: -4px 0 14px;
    color: var(--text-muted);
  }

  .card {
    padding: 14px 16px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface);
  }

  .row {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .grow {
    display: flex;
    flex: 1;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px 10px;
    min-width: 0;
  }

  .avatar {
    display: grid;
    flex-shrink: 0;
    place-items: center;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--accent-gradient);
    color: #fff;
    font-size: 18px;
    font-weight: 800;
  }

  .username {
    overflow: hidden;
    font-weight: 700;
    text-overflow: ellipsis;
  }

  .services {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 10px;
  }
</style>
