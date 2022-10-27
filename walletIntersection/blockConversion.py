import requests

API_KEY = ''

def blockNumber(time, place):
    bp = {
        "module": 'block',
        "action": 'getblocknobytime',
        "timestamp": time,
        "closest": place,
        "apikey": API_KEY
    }

    response = requests.get('https://api.etherscan.io/api?', params=bp)
    response.raise_for_status()
    block = response.json()['result']
    print(block)
    return block
