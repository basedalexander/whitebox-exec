const RustRunnerApiService = require('./rust-runner-api.serivce');

class RustCodeRunner {
    apiService = new RustRunnerApiService();
    isCompiled = false;

    async run(code, params, data) {
        if (!this.isCompiled) {
            await this.compile(code, params, data);
        }

        const runResult = await this.apiService.run(code, params, data);

        return {
            result: runResult.result,
            proof: runResult.proof
        }
    };

    async compile(code, params, data) {
       const result = await this._compile(code, params, data);
       
       if (result) {
        this.isCompiled = true;
       } else {
        this.isCompiled = false;
       }
    }

    async _compile(code, params, data) {
        try {
            await this.apiService.compile(code, params, data)
            return true;
        } catch (e) {
            return false;
        }
    }
}

module.exports = RustCodeRunner;