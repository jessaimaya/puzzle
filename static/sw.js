const staticCache = 'static-cache';
const dynamicCache = 'dynamic-cache';
const assets = [
    '/',
    '/index.html',
    '/main.js',
    '/main.wasm'
];

self.addEventListener('install', event => {
    return event.waitUntil(
        caches.open(staticCache).then(cache => {
           return cache.addAll(assets)
        })
    );
});

self.addEventListener('activate', event => {
    console.log('ServiceWorker activated');
});

self.addEventListener('fetch', event => {
    event.respondWith(
        caches.match(event.request).then(staticResponse => {
            return staticResponse || fetch(event.request).then(dynamicResponse => {
                return caches.open(dynamicCache).then(cache => {
                    cache.put(event.request.url, dynamicResponse.clone());
                    return dynamicResponse;
                })
            })
        })
    )
});