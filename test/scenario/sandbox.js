const { mainConfig } = require("../config");

module.exports = scenario => {
  scenario("update parent with child attached", async (s, t) => {
    const { alice } = await s.players({
      alice: mainConfig
    });

    await alice.spawn();

    // create parent
    const createParent = await alice.call(
      "sandbox",
      "sandbox",
      "create_my_parent_entry",
      {
        entry: {
          content: "fooParentContent"
        }
      }
    );
    t.ok(createParent.Ok);

    // create child
    const createChild = await alice.call(
      "sandbox",
      "sandbox",
      "create_my_child_entry",
      {
        parent: createParent.Ok,
        entry: {
          content: "fooChildContent"
        }
      }
    );
    t.ok(createChild.Ok);

    await s.consistency();

    // get links
    const getLinks = await alice.call("sandbox", "sandbox", "get_children", {
      address: createParent.Ok
    });
    t.ok(getLinks.Ok);
    t.deepEqual(getLinks.Ok.links.length, 1);

    // update parent
    const updateParent = await alice.call(
      "sandbox",
      "sandbox",
      "update_my_parent_entry",
      {
        address: createParent.Ok,
        entry: {
          content: "barParentContent"
        }
      }
    );
    t.ok(updateParent.Ok);

    await s.consistency();

    // get links again
    const getLinksAgain = await alice.call(
      "sandbox",
      "sandbox",
      "get_children",
      { address: createParent.Ok }
    );
    t.ok(getLinksAgain.Ok);
    t.deepEqual(getLinksAgain.Ok.links.length, 1);

    // get links again with update address
    const getLinksAgainWithUpdateAddress = await alice.call(
      "sandbox",
      "sandbox",
      "get_children",
      { address: updateParent.Ok }
    );
    t.ok(getLinksAgainWithUpdateAddress.Ok);
    t.deepEqual(getLinksAgainWithUpdateAddress.Ok.links.length, 1); // not ok
  });
};
