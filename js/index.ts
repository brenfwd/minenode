import minenode from "../crates/minenode_napi/index.js";

console.log("Launching Minenode Server from JS...");
await minenode.runServer({
    host: "0.0.0.0",
    port: 25565,
});
console.log("Minenode Server returned to JS.");
