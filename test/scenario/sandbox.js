const { mainConfig } = require("../config");

module.exports = scenario => {
  scenario("get entry history", async (s, t) => {
    const { alice } = await s.players({
      alice: mainConfig
    });

    await alice.spawn();

    // create entry
    const createMyEntry = await alice.call(
      "sandbox",
      "sandbox",
      "create_my_entry",
      {
        entry: {
          content: "fooContent"
        }
      }
    );
    t.ok(createMyEntry.Ok);

    await s.consistency();

    // update entry
    const updateMyEntry = await alice.call(
      "sandbox",
      "sandbox",
      "update_my_entry",
      {
        address: createMyEntry.Ok,
        entry: {
          content: "barContent"
        }
      }
    );
    t.ok(updateMyEntry.Ok);

    await s.consistency();

    const getOriginalMyEntry = await alice.call(
      "sandbox",
      "sandbox",
      "get_original_address",
      {
        address: updateMyEntry.Ok
      }
    );
    t.ok(getOriginalMyEntry.Ok);
    t.notEqual(getOriginalMyEntry.Ok, updateMyEntry.Ok);
    t.equal(getOriginalMyEntry.Ok, createMyEntry.Ok);
  });
};
