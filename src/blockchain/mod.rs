pub mod veiculo;

use crate::blockchain::veiculo::Veiculo;

use std::io::prelude::*;
use std::fs::{self, DirEntry, File};
use std::path::Path;

use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Report {
    id_prestador: String,
    id_veiculo: String,
    timestamp: String,
    chasis: String,
    km: i64,
    relatorio: String,
    assinatura: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    nmr: i64,
    prev_hash: String,
    this_hash: String,
    reports: Vec<Report>,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    nmr_ultimo_bloco: String,
    db: Vec<Veiculo>,
    bc: Vec<Block>,
}

pub fn inicializar_db() -> Blockchain {
    let db_path = Path::new("./db");

    let mut blocos: Vec<Block> = Vec::new();
    
    for entry in fs::read_dir(db_path).unwrap() {
        let mut entry = File::open(entry.unwrap().path()).unwrap();

        let mut contents = String::new();
        entry.read_to_string(&mut contents).unwrap();
        let block = json::parse(&contents).unwrap();

        let mut reports: Vec<Report> = Vec::new();

        for x in block["reports"].members() {
            let y = Report {
                id_prestador: x["id_prestador"].to_string(),
                id_veiculo: x["id_veiculo"].to_string(),
                timestamp: x["timestamp"].to_string(),
                chasis: x["chasis"].to_string(),
                km: x["km"].as_i64().unwrap(),
                relatorio: x["relatorio"].to_string(),
                assinatura: x["assinatura"].to_string(),
            };

            reports.push(y);
        }

        let block = Block {
            nmr: block["nmr"].as_i64().unwrap(),
            prev_hash: block["prev_hash"].to_string(),
            this_hash: block["this_hash"].to_string(),
            reports,
        };

        blocos.push(block);

    }

    blocos.sort_by(|a, b| a.nmr.partial_cmp(&b.nmr).unwrap());

    let mut p = blocos.get(0).unwrap().clone();
    let q = blocos.get(1..).unwrap().clone();

    for (i, x) in q.iter().enumerate() {
        if i == 0 {continue};

        if x.prev_hash != p.this_hash {
            panic!("Blocos falharam em validar");
        } else {
            p = x.clone()
        }
    
    }

    println!("Blocos validados com sucesso!");

    let mut v: Vec<Veiculo> = Vec::new();

    for b in blocos.clone() {
        for r in b.reports {
            let id_veiculo = r.clone().id_veiculo;

            let mut contem = false;

            for cv in v.iter_mut() {
                if cv.id == id_veiculo {
                    contem = true;
                    cv.relatorios.push(r.clone());
                } else {
                }
            }
            if contem == false {
                v.push(Veiculo {
                    id: id_veiculo.clone(),
                    chasis: r.clone().chasis,
                    km_atual: 0,
                    relatorios: vec![r.clone()],
                });
            }
        }
        
    }

    for x in v.iter_mut() {
        x.verificar();
    }

    let last_block = blocos.get(blocos.len() - 1).unwrap().clone().this_hash;

    Blockchain {
        nmr_ultimo_bloco: last_block,
        db: v,
        bc: blocos,
    }
    
}


