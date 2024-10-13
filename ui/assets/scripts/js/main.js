const invoke = window.__TAURI__.core.invoke;  //  DO NOT REMOVE!!

// Invoke the command
invoke('hello')
    .then(r => console.log(r))
    .catch(e => console.error(e))
