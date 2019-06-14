# BlockCar Genesis

### Modelagem de dados
- Blockchain (private net, futuramente foundation net?)
	- Todos os processos registrados na blockchain precisam iniciar com, respectivamente:
		- Endereço do servidor
		- Endereço do veículo
		- Número único do serviço
	- Todos os outros dados são criptografados com a chave publica
	- Podem ser descriptografados com a chave privada se o proprietário quiser
- Banco de dados (nosso)
	- Dados que o proprietário atual do veiculo quer deixar público para consulta
	- Dados sobre o proprietário

#### Endereços dos contratos
- 0xee6f3e4135647f7b07012706f0df54d23d04e7b625ae5c33f44d8ee616889b7b (0.1)

#### Objetivo
- Construir uma api basica, com as funcionalidades necessárias para o funcionamento
- Requisitos
	- [ ] Submeter dados para a blockchain
	- [ ] Proprietário assinar estes dados utilizando SmartContracts
	- [ ] Criptografar dados e fechar o bloco
	- [ ] Buscar dados do veículo mediante autorização do proprietário e retornar por meio de uma API a ser urilizada no site
