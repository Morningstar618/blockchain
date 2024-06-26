# Module 1 - Create Blockchain

# Importing the Libraries
import datetime
import hashlib
import json
from flask import Flask, jsonify

# Part 1 - Building a Blockchain

class Blockchain:
    
    def __init__(self):
        self.chain = []
        self.create_block(proof = 1, previous_hash = "0")
        
    def create_block(self, proof, previous_hash):
        block = {'index': len(self.chain) + 1,
                 'timestamp': str(datetime.datetime.now()),
                 'proof': proof,
                 'previous_hash': previous_hash}
        self.chain.append(block)
        return block

    def get_previous_block(self):
        return self.chain[-1]
        
    def proof_of_work(self, previous_proof):
        new_proof = 1
        check_proof = False
        while check_proof is False:
            proof = hashlib.sha256(str(new_proof**2 - previous_proof**2)
                                   .encode()).hexdigest()
            if proof[:4] == "0000":
                check_proof = True
            else:
                new_proof += 1
        return new_proof
    
    def get_block_hash(self, block):
        encoded_block = json.dumps(block, sort_keys=True).encode()
        return hashlib.sha256(encoded_block).hexdigest()
    
    def is_chain_valid(self, chain):
        previous_block = chain[0]
        block_index = 1
        while block_index < len(chain):
            block = chain[block_index]
            if self.get_block_hash(previous_block) != block['previous_hash']:
                return False
            new_proof = block['proof']
            previous_proof = previous_block['proof']
            proof_hash_operation = hashlib.sha256(str(new_proof**2 - previous_proof**2)
                                   .encode()).hexdigest()
            if proof_hash_operation[:4] != '0000':
                return False
            previous_block = block
            block_index += 1
        return True        
       
        
# Part 2 - Mining our Blockchain

# Creating a Web App
app = Flask(__name__)

# Creating a Blockchain
blockchain = Blockchain()

# Mine a new block
@app.route('/mine_block', methods = ['GET'])
def mine_block():
    previous_block = blockchain.get_previous_block()
    previous_proof = previous_block['proof']
    proof = blockchain.proof_of_work(previous_proof)
    previous_block_hash = blockchain.get_block_hash(previous_block)
    block = blockchain.create_block(proof, previous_block_hash)
    response = {'message': 'Congratulations, you have mined a block',
                'index': block['index'],
                'timestamp': block['timestamp'],
                'proof': block['proof'],
                'previous_hash': block['previous_hash']}
    return jsonify(response), 200

# Get the full chain
@app.route('/get_chain', methods = ['GET'])
def get_chain():
    response = {'chain': blockchain.chain,
                'len': len(blockchain.chain)}
    return jsonify(response), 200

# Check if chain is valid
@app.route('/is_valid', methods = ['GET'])
def is_valid():
    chain = blockchain.chain
    valid = blockchain.is_chain_valid(chain)
    response = {'is blockchain valid': valid}
    return jsonify(response), 200

# Running the app
app.run(host='127.0.0.1', port='5000')







