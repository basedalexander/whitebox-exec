const Executor = require('./executor/executor.js');
const model = require('./executor/mocks/algo.mock.json');
const publicationsResult = require('./executor/mocks/publications.mock.json');

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