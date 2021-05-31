// Debug server mode.
// This mode is designed for rapid development. This includes using 'localhost:9090' as the api target instead of
// wherever this application is served from.

export const environment = {
  production: false,
  serve: true
};

import 'zone.js/plugins/zone-error';
