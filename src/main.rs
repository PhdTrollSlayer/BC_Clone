#![feature(proc_macro_hygiene, decl_macro)]
mod blockchain;
mod criptografia;
mod models;

use blockchain::*;
use models::report::*;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::http::RawStr;
use rocket::response::content::Json;

#[allow(deprecated, unreachable_code)]
fn main() {
    let mut bc = Blockchain::inicializar();

    let r = Report {
        id_prestador: "89".to_string(),
        id_veiculo: "3213".to_string(),
        timestamp: "12312312".to_string(),
        chasis: "123123".to_string(),
        km: 0,
        relatorio: "".to_string(),
        assinatura: "123".to_string(),
    };

    rocket::ignite()
           .mount("/", routes![consultar_placa])
           .manage(bc)
           .launch();
}

#[get("/consulta/<placa>")]
fn consultar_placa(bc: State<Blockchain>, placa: &RawStr) -> Json<String> {
    let resultado = bc.consultar_veiuculo(placa);

    match resultado {
        Some(v) => {
            Json(v.json_data())
        },
        None => {Json("{}".to_string())}
    }
}
