const test = require('node:test');

test('synchronous passing test', (_t) => {
	//This test passes because it does not throw an exception.
	assert.strictEqual(1, 1);
});


//TODO: test for Promise.Ill need it later
//
// test('failing test using Promises', (t) => {
//   // Promises can be used directly as well.
//   return new Promise((resolve, reject) => {
//     setImmediate(() => {
//       reject(new Error('this will cause the test to fail'));
//     });
//   });
// });
