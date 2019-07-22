mod blockchain;
mod criptografia;

use blockchain::*;

/* Notas
*/

#[allow(deprecated, unreachable_code)]
fn main() {

    let mut bc = Blockchain::inicializar_db();

    let r = Report {
        id_prestador: "1".to_string(),
        id_veiculo: "2".to_string(),
        timestamp: "1563799664".to_string(),
        chasis: "46548".to_string(),
        km: 100,
        relatorio: "to".to_string(),
        assinatura: "".to_string(),
    };

    let v = vec![r];

    bc.inserir_bloco(&v);
}
