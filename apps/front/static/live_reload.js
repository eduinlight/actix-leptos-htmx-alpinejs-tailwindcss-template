function connect() {
  const ws = new WebSocket('/ws-live-reload')
  const LIVE_RELOAD_SERVER_VERSION_KEY="live-reload-server-version"

  ws.addEventListener('open', () => {
    ws.send("")
    console.log("Live reload socket is open")
  })

  ws.addEventListener('message', (message) => {
    const serverVersion = message.data
    const previousVersion = localStorage.getItem(LIVE_RELOAD_SERVER_VERSION_KEY)

    localStorage.setItem(LIVE_RELOAD_SERVER_VERSION_KEY, serverVersion) 

    if(serverVersion !== previousVersion){
      htmx.ajax('GET', location.pathname, {
        target: "#root",
        swap: "innerHTML"
      })
    }
  })

  ws.addEventListener('close', (e) => {
    console.log('Live reload socket is closed. Reconnect will be attempted in 1 second.', e.reason)
    setTimeout(function() {
      connect()
    }, 1000)
  })

  ws.addEventListener('error', (err) => {
    console.error('Live reload socket encountered error: ', err?.message ?? "unknown.", 'Closing socket')
    ws.close()
  })
}

connect()
