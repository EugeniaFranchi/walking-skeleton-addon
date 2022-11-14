const express = require("express");

let counter = {};
const app = express();

app.get("/", (req, res) => {
    res.send("Hello world");
});

app.get("/counter", (req, res) => {
    counter[req.site] = counter[req.site] + 1 || 0;
    res.send(counter);
});

app.listen(3000, () => console.log('Server listening at port 3000'));