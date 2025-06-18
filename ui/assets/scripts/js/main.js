const invoke = window.__TAURI__.core.invoke;  // DO NOT REMOVE!!

document.addEventListener('DOMContentLoaded', () => {
    document.querySelector('button.click-me').addEventListener('click', () => {
        invoke("hello", { name: "Developer" })
            .then(msg => {
                document.querySelector("h1.title").textContent = msg;
                console.log(msg);
            })
            .catch(e => console.error(e))
    });
});
