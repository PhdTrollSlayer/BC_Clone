use web3::futures::Future;
use web3::contract::Contract;

fn deploy() {
    let (_eloop, transport) = web3::transports::Http::new("http://127.0.0.1:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    let accounts = web3.eth().accounts().wait().unwrap();

    let end_servidor = accounts[0];
    let end_veiculo = accounts[1];
    // Bytecode do contrato
    let bytecode = include_str!("../../solidity/store.code");
    
    // Depoly do contrato para a net
    let contract = Contract::deploy(web3.eth(), include_bytes!("../../solidity/store.abi")).unwrap()
        .confirmations(0)
        .execute(bytecode, (), end_servidor)
        .unwrap()
        .wait()
        .unwrap();

    println!("{:?}", contract.address());
}
