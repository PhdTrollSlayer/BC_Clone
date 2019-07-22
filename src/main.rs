#![feature(proc_macro_hygiene, decl_macro)]
mod blockchain;
mod criptografia;

use blockchain::*;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::http::RawStr;
use rocket::response::content::Json;

#[allow(deprecated, unreachable_code)]
fn main() {
    let mut bc = Blockchain::inicializar_db();

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
