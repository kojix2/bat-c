const addon = require("./build/Release/self_print.node");
const result = addon.selfPrint();
process.stdout.write(result);
