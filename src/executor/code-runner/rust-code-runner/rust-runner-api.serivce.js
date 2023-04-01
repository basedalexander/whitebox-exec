class RustRunnerApiService {
    async compile(code, params, data) {
        const response = await fetch(`http://localhost:4000/compile`, {
          method: 'POST',
          body: JSON.stringify({
            code,
            params,
            data
          })
        });
        const result = await response.json();
        return result;
      }

      async run(code, params, data) {
        const response = await fetch(`http://localhost:4000/run`, {
          method: 'POST',
          body: JSON.stringify({
            code,
            params,
            data
          })
        });
        const result = await response.json();
        return result;
    }
}

module.exports = RustRunnerApiService;
  
