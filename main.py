from pprint import pprint
from web3 import Web3, HTTPProvider

web3 = Web3(Web3.WebsocketProvider("ws://127.0.0.1:8546"))

pprint(Web3.toJSON(web3.eth.getBlock('latest')))
