use crate::models::veiculo::Veiculo;
use crate::models::report::Report;
use crate::models::prestador::Prestador;

use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;
use std::time::{Duration, SystemTime};

use openssl::sha;

use serde::{Deserialize, Serialize};

use chrono::prelude::*;

const BC_PATH: &str = "./blockchain";
const DB_PATH: &str = "./db";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    nmr:       i64,
    prev_hash: String,
    this_hash: String,
    reports:   Vec<Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    intervalo_att:     Duration,
    ultima_att:        SystemTime,
    fila:              Vec<Report>,
    nmr_ultimo_bloco:  i64,
    hash_ultimo_bloco: String,
    prestador_db:      Vec<Prestador>,
    veiculo_db:        Vec<Veiculo>,
    bc:                Vec<Block>,
}

impl Blockchain {
    pub fn confirm_api_key(&self, q: &str) -> Result<(String),(())> {
        for p in self.prestador_db.iter() {
            if q == p.api_key {
                return Ok(serde_json::to_string(&p).unwrap())
            }
        }

        Err(())
    }

    pub fn get_all_prestadores(&self) -> String {
        let mut p = self.prestador_db.clone();

        for i in p.iter_mut() {
            i.veiculos_presentes = Vec::new();
        }

        serde_json::to_string(&p).unwrap()
    }

    pub fn get_all_veiculos(&self) -> String {
        let mut v = self.veiculo_db.clone();

        for i in v.iter_mut() {
            i.relatorios = Vec::new();
        }

        serde_json::to_string(&v).unwrap()
    }

    pub fn inserir_report(&mut self, r: Report) {
        self.fila.push(r);

        if self.ultima_att.elapsed().unwrap() >= self.intervalo_att {
            self.push_block();

            self.ultima_att = SystemTime::now();
        }
    }

    pub fn consultar_veiuculo(&self, query: &str) -> Option<Veiculo> {
        for v in self.veiculo_db.clone() {
            if &v.id == query {
                return Some(v.clone());
            }
        }

        None
    }

    pub fn inicializar() -> Blockchain {
        let mut blocos: Vec<Block> = Vec::new();
        
        for entry in fs::read_dir(Path::new(BC_PATH)).unwrap() {
            let mut entry = File::open(entry.unwrap().path()).unwrap();

            let mut contents = String::new();
            entry.read_to_string(&mut contents).unwrap();
            let block = json::parse(&contents).unwrap();

            let mut reports: Vec<Report> = Vec::new();

            for x in block["reports"].members() {
                let y = Report {
                    id_prestador: x["id_prestador"].to_string(),
                    id_veiculo:   x["id_veiculo"].to_string(),
                    timestamp:    x["timestamp"].to_string(),
                    chasis:       x["chasis"].to_string(),
                    km:           x["km"].as_i64().unwrap(),
                    relatorio:    x["relatorio"].to_string(),
                    assinatura:   x["assinatura"].to_string(),
                };

                reports.push(y);
            }

            let block = Block {
                nmr:       block["nmr"].as_i64().unwrap(),
                prev_hash: block["prev_hash"].to_string(),
                this_hash: block["this_hash"].to_string(),
                reports,
            };

            blocos.push(block);

        }
        
        let mut prestadoras: Vec<Prestador> = Vec::new();
        let mut i = 0;

        for entry in fs::read_dir(Path::new(DB_PATH)).unwrap() {
            let mut entry = File::open(entry.unwrap().path()).unwrap();

            let mut contents = String::new();
            entry.read_to_string(&mut contents).unwrap();
            let prestadora = json::parse(&contents).unwrap();

            let mut v_presentes: Vec<Veiculo> = Vec::new();

            for x in prestadora["veiculos_presentes"].members() {
                let x: Veiculo = serde_json::from_str(&x.dump()).unwrap();
                v_presentes.push(x);
            }

            prestadoras.push(
                Prestador {
                    nome: prestadora["nome"].to_string(),
                    api_key: prestadora["api_key"].to_string(),
                    veiculos_presentes: v_presentes,
                }
            );

            i += 1;

        }

        println!("{} prestadoras de serviço cadastradas!", i);

        let mut bc = Blockchain {
            intervalo_att:     Duration::new(15, 0),
            ultima_att:        SystemTime::now(),
            fila:              Vec::new(),
            nmr_ultimo_bloco:  0,
            hash_ultimo_bloco: String::new(),
            prestador_db:      prestadoras,
            veiculo_db:        Vec::new(),
            bc:                blocos,
        };

        bc.validar_blocos();
        
        let hash_last_block = bc.bc.get(bc.bc.len() - 1).unwrap().clone().this_hash;
        let nmr_last_block = bc.bc.get(bc.bc.len() - 1).unwrap().clone().nmr;

       
        bc.validar_veiculos();
        
        bc.nmr_ultimo_bloco = nmr_last_block;
        bc.hash_ultimo_bloco = hash_last_block;

        bc
        
    }

    pub fn cadastrar_prestadora(&mut self, nome: &str) {
        let p = Prestador::new(nome);

        self.prestador_db.push(p.clone());

        let s = serde_json::to_string(&p).unwrap();

        let mut file = File::create(format!("{}/b{}.json", DB_PATH, p.api_key)).unwrap();
        file.write_all(s.as_bytes()).unwrap();
        
    }

    fn validar_blocos(&mut self) {
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

    fn validar_veiculos(&mut self) {
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

        self.veiculo_db = v;
    }

    pub fn push_block(&mut self) {
        let nmr_novo_bloco = self.nmr_ultimo_bloco + 1;
        let hash_ultimo_bloco = self.hash_ultimo_bloco.clone();

        let mut f_string = String::new();

        for r in self.fila.iter_mut() {
            f_string.push_str(
                &format!("{}{}{}{}{}{}", 
                         r.id_prestador, 
                         r.id_veiculo, 
                         r.timestamp, 
                         r.km, 
                         r.relatorio, 
                         self.hash_ultimo_bloco.clone()));
        }

        let mut hasher = sha::Sha256::new();

        hasher.update(f_string.as_bytes());

        let this_hash = hex::encode(hasher.finish());

        let novo_bloco = Block {
            nmr:       nmr_novo_bloco,
            prev_hash: hash_ultimo_bloco,
            this_hash,
            reports:   self.fila.clone(),
        };
        let mut r: Vec<_> = Vec::new();

        for report in novo_bloco.reports.clone() {
            r.push(report.clone());
        }

        let s = serde_json::to_string(&novo_bloco).unwrap();

        self.bc.push(novo_bloco.clone());
        self.validar_blocos();
        self.validar_veiculos();
        
        let mut file = File::create(format!("{}/b{}.json", BC_PATH, novo_bloco.nmr)).unwrap();
        file.write_all(s.as_bytes()).unwrap();

    }

}
