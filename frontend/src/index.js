module.exports = import('./frontend.wasm').then(function ({add_two}) {
  console.log("3 + 5 = " + add_two(3, 5));

  return add_two(2, 3);
});

console.log("something" + model.exports);
