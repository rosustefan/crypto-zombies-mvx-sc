use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/crypto-zombies.mxsc.json", crypto_zombies::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/crypto_zombies.scen.json");
}
