const addon = require("./build/Release/basic.node");
const result = addon.basic();
process.stdout.write(result);
