// For Node.js
var TurndownService = require('turndown')
const fs = require("fs");

var turndownService = new TurndownService({
    headingStyle: "atx",
    hr: "---",
    bulletListMarker: "-",
    codeBlockStyle: "fenced",
})

process.stdin.resume();
process.stdin.setEncoding('utf8');
process.stdin.on('data', function(data) {
  process.stdout.write(turndownService.turndown(data));
});