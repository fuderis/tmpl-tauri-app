const invoke = window.__TAURI__.core.invoke;  //  DO NOT REMOVE!!

// Invoke the command
invoke('my_custom_command')
    .then(res => console.log(res))
    .catch(e => console.error(e))
