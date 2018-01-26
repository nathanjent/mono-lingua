module.exports = import('./frontend.wasm').then(function ({add}) {
  console.log("3 + 5 = " + add(3, 5));

  return add(2, 3);
});
