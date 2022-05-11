const jayson = require('jayson');

const callClient = jayson.Client.http('http://127.0.0.1:3000/rpc');

callClient.request('hello', ['114', 514, { 1919: 810 }], (err, result) => {
  console.log(err, result);
});
