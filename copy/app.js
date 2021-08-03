const express = require("express")
const path = require("path")

const PORT = ;
const app = express()

app.use(express.static(path.join(__dirname, "build")))

app.listen(PORT, () => console.log(`Server running on port ${PORT}`))