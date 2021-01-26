import('./pkg/html').then(m=>{
    window.my_function = m.my_function;
}).catch(e => console.error("Error importing index.js:", e));