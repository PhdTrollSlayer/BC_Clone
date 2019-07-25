# BlockCar Genesis

### API

##### Consulta de Veiculos
* GET
* ```/consulta/<placa>```
* Retorno
```json
{
  "id": "1",
  "chasis": "46548",
  "km_atual": 0,
  "relatorios": [
    {
      "id_prestador": "00",
      "id_veiculo": "1",
      "timestamp": "1563162327",
      "chasis": "46548",
      "km": 10000,
      "relatorio": "",
      "assinatura": "444444444"
    },
    {
      "id_prestador": "00",
      "id_veiculo": "1",
      "timestamp": "1563562327",
      "chasis": "46548",
      "km": 40000,
      "relatorio": "",
      "assinatura": "444444444"
    }
  ]
}
```

### TODO
- [x] Blockchain funcional
- [x] Consultar veiculos
- [ ] Submeter relatórios
- [ ] Preparar API
- [ ] Site
	- [ ] Login
	- [ ] Credenciais 
	- [ ] Multiassinatura nos relatórios
	- [ ] Consulta
