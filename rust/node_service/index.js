const fs = require('node:fs');
const {get_table}= require('./pkg/table_task_with_js');
const { table } = require('node:console');


global.readData = function readData() {
  try {
    const table_data = fs.readFileSync('./files/data.json', 'utf8');
    return JSON.stringify(JSON.parse(table_data));
  } catch (err) {
    console.error('Error reading data:', err);
    throw err;
  }
}

// console.log(readData());

try {
  console.log(get_table());
}catch(e) {
  console.log(e)
}


