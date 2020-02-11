const { Config } = require("@holochain/tryorama");

const sandbox = Config.dna("dist/sandbox.dna.json", "sandbox");

const mainConfig = Config.gen(
  {
    sandbox: sandbox
  },
  {
    logger: Config.logger({ type: "error" })
  }
);

module.exports = {
  mainConfig
};
