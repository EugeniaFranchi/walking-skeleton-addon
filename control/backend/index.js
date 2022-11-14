const express = require("express");
const cors = require('cors');

let counter = {};
const app = express();
app.use(express.json());
app.use(cors());

app.get("/", (req, res) => {
    res.send("Hello world");
});

app.get("/counter", (req, res) => {
    res.send(counter);
});

app.post("/counter", (req, res) => {
    console.log(req.body);
    counter[req.body.site] = { stego: req.body.stego, total: req.body.total }
    res.send(counter);
});

app.listen(5000, () => console.log('Server listening at port 5000'));