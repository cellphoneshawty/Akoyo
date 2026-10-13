export type Route = 'search' | 'liked' | 'settings';

const routes: Route[] = ['search', 'liked', 'settings'];

function parse(): Route {
  const name = location.hash.replace(/^#\/?/, '');
  return routes.find((route) => route === name) ?? 'search';
}

let current = $state<Route>(parse());
window.addEventListener('hashchange', () => (current = parse()));

export const router = {
  get route() {
    return current;
  },
};

export const href = (route: Route) => `#/${route}`;
