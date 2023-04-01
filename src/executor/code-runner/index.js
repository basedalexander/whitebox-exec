const RustCodeRunner = require('./rust-code-runner/rust.code-runner');

const codeRunnerRegistry = {
    "rust": new RustCodeRunner()
}

module.exports = codeRunnerRegistry;