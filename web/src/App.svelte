<script lang="ts">
  import { onMount } from 'svelte';
  import Logo from './components/Logo.svelte';
  import { t } from './lib/i18n.svelte';
  import { bootstrap, session } from './lib/session.svelte';
  import AppShell from './screens/AppShell.svelte';
  import AuthScreen from './screens/AuthScreen.svelte';

  onMount(bootstrap);
</script>

{#if !session.ready}
  <div class="center"><Logo size={56} /></div>
{:else if session.offline}
  <div class="center">
    <Logo size={56} />
    <h1>{t('offline.title')}</h1>
    <p>{t('offline.text')}</p>
    <button class="btn btn-primary" onclick={bootstrap}>{t('offline.retry')}</button>
  </div>
{:else if session.user}
  <AppShell user={session.user} />
{:else}
  <AuthScreen />
{/if}

<style>
  .center {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
    height: 100%;
    padding: 16px;
    text-align: center;
  }

  h1 {
    font-size: 24px;
  }

  p {
    margin: 0 0 6px;
    color: var(--text-muted);
  }
</style>
