mod blockchain;
mod criptografia;

use criptografia::*;

use std::process::exit;

/* Notas
*/

#[allow(deprecated, unreachable_code)]
fn main() {

    let bc = blockchain::inicializar_db();
}
