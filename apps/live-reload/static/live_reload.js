function connect() {
  const liveReloadServer = "http://localhost:4000/ws"
  const ws = new WebSocket(liveReloadServer)

  ws.addEventListener('open', () => {
    ws.send("")
    console.log("Live reload socket is open")
  })

  ws.addEventListener('message', (message) => {
    htmx.ajax('GET', location.pathname, {
      target: "#root",
      swap: "innerHTML"
    })
  })

  ws.addEventListener('close', (e) => {
    console.log('Live reload socket is closed. Reconnect will be attempted in 1 second.', e.reason)
    setTimeout(function() {
      connect()
    }, 400)
  })

  ws.addEventListener('error', (err) => {
    console.error('Live reload socket encountered error: ', err?.message ?? "unknown.", 'Closing socket')
    ws.close()
  })
}

connect()
