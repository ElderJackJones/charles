const { PowerShell } = require('node-powershell');

PowerShell.$`echo testing > testing.txt`;
console.log('this far')