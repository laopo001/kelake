import('./pkg/html').then(m=>{
    window.call_task = m.call_task;
}).catch(e => console.error("Error importing index.js:", e));