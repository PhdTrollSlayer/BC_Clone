pub mod veiculo;

use crate::blockchain::veiculo::Veiculo;

use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

use openssl::sha;

use json::*;

const DB_PATH: &str = "./blockchain";

#[derive(Debug, Clone)]
pub struct Report {
    pub id_prestador: String,
    pub id_veiculo: String,
    pub timestamp: String,
    pub chasis: String,
    pub km: i64,
    pub relatorio: String,
    pub assinatura: String,
}

impl Report {
    fn json(&self) -> String {
        let data = object!{
                "id_prestador" => self.id_prestador.clone(),
                "id_veiculo" => self.id_veiculo.clone(),
                "timestamp" => self.timestamp.clone(),
                "chasis" => self.chasis.clone(),
                "km" => self.km.clone(),
                "relatorio" => self.relatorio.clone(),
                "assinatura" => self.assinatura.clone()
        };

        data.dump()
    }
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
    nmr_ultimo_bloco: i64,
    hash_ultimo_bloco: String,
    db: Vec<Veiculo>,
    bc: Vec<Block>,
}

impl Blockchain {
    pub fn consultar_veiuculo(&self, query: &str) -> Option<Veiculo> {
        for v in self.db.clone() {
            if &v.id == query {
                return Some(v.clone());
            }
        }

        None
    }

    pub fn inicializar_db() -> Blockchain {
        let mut blocos: Vec<Block> = Vec::new();
        
        for entry in fs::read_dir(Path::new(DB_PATH)).unwrap() {
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

        let mut bc = Blockchain {
            nmr_ultimo_bloco: 0,
            hash_ultimo_bloco: String::new(),
            db: Vec::new(),
            bc: blocos,
        };

        bc.validar_blocos();
        
        let hash_last_block = bc.bc.get(bc.bc.len() - 1).unwrap().clone().this_hash;
        let nmr_last_block = bc.bc.get(bc.bc.len() - 1).unwrap().clone().nmr;

       
        bc.validar_veiculos();
        
        bc.nmr_ultimo_bloco = nmr_last_block;
        bc.hash_ultimo_bloco = hash_last_block;

        bc
        
    }

    pub fn validar_blocos(&mut self) {
        self.bc.sort_by(|a, b| a.nmr.partial_cmp(&b.nmr).unwrap());

        let q = self.bc.get(1..).unwrap().clone();

        for (i, x) in q.iter().enumerate() {
            if i == 0 {continue};
            
            let prev = self.bc.get(i).unwrap().clone();

            if x.prev_hash != prev.this_hash {
                panic!("Blocos falharam em validar");
            } 
        
        }

        println!("Blocos validados com sucesso!");

    }

    pub fn validar_veiculos(&mut self) {
        let mut v: Vec<Veiculo> = Vec::new();

        for b in self.bc.clone() {
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

        self.db = v;
    }

    pub fn inserir_bloco(&mut self, reports: &Vec<Report>) {
        let nmr_novo_bloco = self.nmr_ultimo_bloco + 1;
        let hash_ultimo_bloco = self.hash_ultimo_bloco.clone();

        let mut f_string = String::new();

        for r in reports {
            f_string.push_str(&format!("{}{}{}{}{}{}{}", r.id_prestador, r.id_veiculo, r.timestamp, r.chasis, r.km, r.relatorio, r.assinatura));
        }

        let mut hasher = sha::Sha256::new();

        hasher.update(f_string.as_bytes());

        let this_hash = hex::encode(hasher.finish());

        let novo_bloco = Block {
            nmr: nmr_novo_bloco,
            prev_hash: hash_ultimo_bloco,
            this_hash,
            reports: reports.clone(),
        };
        let mut r: Vec<_> = Vec::new();

        for report in novo_bloco.reports.clone() {
            r.push(report.json());
        }

        let s = format!(r#"{}
                            "nmr": {},
                            "prev_hash": "{}",
                            "this_hash": "{}",
                            "reports": {}
                            {}"#,
                            '{',
                            novo_bloco.nmr,
                            novo_bloco.prev_hash,
                            novo_bloco.this_hash,
                            stringify(r),
                            '}');

        self.bc.push(novo_bloco.clone());
        self.validar_blocos();
        self.validar_veiculos();
        
        let mut file = File::create(format!("{}/b{}.json", DB_PATH, novo_bloco.nmr)).unwrap();
        file.write_all(s.as_bytes()).unwrap();

    }

}
