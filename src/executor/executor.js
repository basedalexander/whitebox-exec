const codeRunnerRegistry = require('./code-runner/index');

class Executor {
    async execute(model, params, data) {
        const lang = model.md.lang;
        const code = model.md.code;

        const runner = this.getCodeRunner(lang);
        const runResult = await runner.run(code, params, data);

        const result = {
            proof: runResult.proof,
            result: runResult.result
        };
        return result;
    }

    getCodeRunner(lang) {
        return codeRunnerRegistry[lang];
    }
}

module.exports = Executor;