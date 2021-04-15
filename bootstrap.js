import './style.scss';

if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js')
        .then(register => console.log('Service worker registered. ', register))
        .catch(error => console.log('Error registering service worker: ', error));
}

import("./pkg").then(module => {
    module.init();
});