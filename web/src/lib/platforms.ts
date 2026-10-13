export type PlatformId = 'youtube_music' | 'soundcloud' | 'spotify' | 'yandex_music' | 'vk_music' | 'apple_music';

export type Platform = {
  id: PlatformId;
  name: string;
  color: string;
  experimental?: boolean;
  available: boolean;
};

export const platforms: Platform[] = [
  { id: 'youtube_music', name: 'YouTube Music', color: '#ff0033', available: false },
  { id: 'soundcloud', name: 'SoundCloud', color: '#ff5500', available: false },
  { id: 'spotify', name: 'Spotify', color: '#1db954', experimental: true, available: false },
  { id: 'yandex_music', name: 'Yandex Music', color: '#ffcc00', available: false },
  { id: 'vk_music', name: 'VK Music', color: '#0077ff', available: false },
  { id: 'apple_music', name: 'Apple Music', color: '#fa2d48', available: false },
];
