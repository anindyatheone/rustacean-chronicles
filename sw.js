const CACHE_NAME = 'rustacean-v1';

// We dynamically cache on the fly to support the Trunk hashed WASM filenames
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => {
            return cache.addAll([
                './',
                './index.html',
                './manifest.json',
                './assets/icon.png',
                // Cutscenes
                './assets/cutscenes/ep1_prologue.png',
                './assets/cutscenes/ep1_outro.png',
                './assets/cutscenes/ep2_intro.png',
                './assets/cutscenes/ep2_outro.png',
                './assets/cutscenes/ep3_intro.png',
                './assets/cutscenes/ep3_outro.png',
                './assets/cutscenes/ep4_intro.png',
                './assets/cutscenes/ep4_outro.png',
                './assets/cutscenes/ep5_intro.png',
                './assets/cutscenes/ep5_outro.png',
                './assets/cutscenes/ep6_intro.png',
                './assets/cutscenes/ep6_outro.png',
                './assets/cutscenes/ep7_intro.png',
                './assets/cutscenes/ep7_outro.png',
                './assets/cutscenes/ep8_intro.png',
                './assets/cutscenes/ep8_outro.png',
                './assets/cutscenes/ep9_intro.png',
                './assets/cutscenes/ep9_epilogue.png'
            ]);
        })
    );
});

// Cache-first strategy for extremely fast offline boots
self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request).then((response) => {
            if (response) {
                return response; // Return from cache directly
            }

            // If not in cache (e.g. the hashed WASM or JS file), fetch from network and cache it
            return fetch(event.request).then(
                (response) => {
                    // Check if we received a valid response
                    if (!response || response.status !== 200 || response.type !== 'basic') {
                        return response;
                    }

                    // Clone the response because streams can only be consumed once
                    var responseToCache = response.clone();

                    caches.open(CACHE_NAME)
                        .then((cache) => {
                            // Don't try to cache API calls or extension requests, just local assets
                            if (event.request.url.startsWith(self.location.origin)) {
                                cache.put(event.request, responseToCache);
                            }
                        });

                    return response;
                }
            ).catch(() => {
                // If network fails (offline) and not in cache, fallback logic (optional)
                console.log("Offline and resource not in cache:", event.request.url);
            });
        })
    );
});

// Purge old caches when versions change
self.addEventListener('activate', (event) => {
    const cacheAllowlist = [CACHE_NAME];
    event.waitUntil(
        caches.keys().then((cacheNames) => {
            return Promise.all(
                cacheNames.map((cacheName) => {
                    if (cacheAllowlist.indexOf(cacheName) === -1) {
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});
