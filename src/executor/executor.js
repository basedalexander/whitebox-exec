class Executor {
    async execute(model, params, data) {
        const result = {
            proof: null,
            paras: params,
            data: data
        };
        return result;
    }
}

module.exports = Executor;