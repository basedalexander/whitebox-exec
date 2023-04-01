const Executor = require('./executor/executor.js');
const model = require('./executor/mocks/algo.mock.json');
const publicationsResult = require('./executor/mocks/publications.mock.json');
const codeMock = require('./executor/mocks/code.mock');

// Settings up mocks
model.md.code = codeMock;
const params = [
    {
        "name": "topics",
        "value": ["nft", "cool"]
    },
    {
        "name": "likesThreshold",
        "value": 500
    }
];
const data = publicationsResult.data.publications.items;

async function main () {
    const executor = new Executor();
    const result = await executor.execute(model, params, data);
    console.log(JSON.stringify(result, null, 4));
}
main();

// todo
// 1. Implement risk0 rust code execution with proof and result (2hours)
// 2. Call rust function from javascript using wasm-bindgen
// 3. Use serialisation/desirealisation to pass complex object to host function.