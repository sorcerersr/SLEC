"use strict";

var cacheName = "slec-v1";
var offlineFundamentals = [
  ".",
  "./index.html",
  "./assets/pico.min.css",
  "./assets/timer.css",
  "./assets/manifest.json",
];

self.addEventListener("install", function (event) {
  event.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(offlineFundamentals);
    })
  );
  self.skipWaiting();
});

self.addEventListener("activate", function (event) {
  event.waitUntil(
    caches.keys().then(function (keys) {
      return Promise.all(
        keys
          .filter(function (key) {
            return key !== cacheName;
          })
          .map(function (key) {
            return caches.delete(key);
          })
      );
    })
  );
  self.clients.claim();
});

self.addEventListener("fetch", function (event) {
  if (event.request.method !== "GET") return;

  event.respondWith(
    caches.match(event.request).then(function (cached) {
      var fetchPromise = fetch(event.request)
        .then(function (response) {
          var clone = response.clone();
          caches.open(cacheName).then(function (cache) {
            cache.put(event.request, clone);
          });
          return response;
        })
        .catch(function () {
          return new Response(
            "<h1>Offline</h1><p>SLEC works offline. Some features may be limited.</p>",
            {
              status: 503,
              statusText: "Service Unavailable",
              headers: new Headers({ "Content-Type": "text/html" }),
            }
          );
        });

      return cached || fetchPromise;
    })
  );
});
