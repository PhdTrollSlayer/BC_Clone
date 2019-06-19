mod contract;
mod criptografia;

use criptografia::*;

use std::process::exit;

use web3::futures::Future;

/* Notas
# Rodar o client do ethereum
parity --config dev --jsonrpc-apis all 

0x00a329c0648769a73afac7f9381e08fb43dbea72 -> tem o dinheiro
*/

#[allow(deprecated, unreachable_code)]
fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://127.0.0.1:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    let current_block = web3.eth().block_number().wait().unwrap();
    println!("Bloco atual: {:#?}", current_block);

    let accounts = web3.eth().accounts().wait().unwrap();

    let end_servidor = accounts[0];
    let end_veiculo = accounts[1];

    let u = web3.personal().unlock_account(end_servidor, "", None).wait().unwrap();
    println!("Account {:?}\nUnlocked: {:?}", end_servidor, u);
    // ************************************************************************

    let end_servidor_saldo = web3.eth().balance(end_servidor, None).wait().unwrap();
    let end_veiculo_saldo = web3.eth().balance(end_veiculo, None).wait().unwrap();

    println!("N1 balance: {:?}", end_servidor_saldo);
    println!("N2 balance: {:?}", end_veiculo_saldo);

    // ***********************************************************************
    
    //gerar_credendiais().unwrap();
    let chave = recuperar_credenciais();

    // ***********************************************************************
    let y = criptografar(&String::from("vai se fude comunismo"), &chave);
    let y = des_criptografar(&y, &chave);
    dbg!(y);



    exit(0);

    /*
    */
}
