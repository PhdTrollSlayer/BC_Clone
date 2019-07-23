use json::*;

#[derive(Debug, Clone)]
pub struct Report {
    pub id_prestador: String,
    pub id_veiculo:   String,
    pub timestamp:    String,
    pub chasis:       String,
    pub km:           i64,
    pub relatorio:    String,
    pub assinatura:   String,
}

impl Report {
    pub fn json_data(&self) -> String {
        let data = object!{
                "id_prestador" => self.id_prestador.clone(),
                "id_veiculo"   => self.id_veiculo.clone(),
                "timestamp"    => self.timestamp.clone(),
                "chasis"       => self.chasis.clone(),
                "km"           => self.km.clone(),
                "relatorio"    => self.relatorio.clone(),
                "assinatura"   => self.assinatura.clone()
        };

        data.dump()
    }
}
