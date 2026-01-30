function sendFunction(func) {
    console.log("Sending function:", func);
    fetch('/api/func', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ func })
    })
    .then(response => response.json())
    .then(data => {
        if (data.status === "success") {
            alert("Function executed successfully");
        } else {
            alert("Error executing function");
        }
    })
}

