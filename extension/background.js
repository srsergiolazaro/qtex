let socket = null;
const SERVER_URL = 'http://localhost:4848';
const WS_URL = 'ws://localhost:4848';

function connect() {
    console.log('Connecting to qtex server...');
    socket = new WebSocket(WS_URL);

    socket.onopen = () => {
        console.log('Connected to qtex server');
    };

    socket.onmessage = (event) => {
        const data = JSON.parse(event.data);
        console.log('Received signal:', data);

        if (data.type === 'reload') {
            handleReload();
        }
    };

    socket.onclose = () => {
        console.log('Disconnected from qtex server. Retrying in 3s...');
        setTimeout(connect, 3000);
    };

    socket.onerror = (err) => {
        console.error('WebSocket error:', err);
        socket.close();
    };
}

async function handleReload() {
    const viewUrl = `${SERVER_URL}/view`;
    const tabs = await chrome.tabs.query({});

    const targetTab = tabs.find(t => t.url && t.url.includes(viewUrl));

    if (targetTab) {
        console.log('Found existing tab, focusing and reloading with cache buster...');
        const busterUrl = `${viewUrl}?t=${Date.now()}`;
        await chrome.tabs.update(targetTab.id, { url: busterUrl, active: true });
        await chrome.windows.update(targetTab.windowId, { focused: true });
    } else {
        console.log('No existing tab found, opening new one...');
        chrome.tabs.create({ url: viewUrl });
    }
}

connect();
