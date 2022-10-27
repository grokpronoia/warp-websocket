import requests
from requests.exceptions import HTTPError
import json

API_KEY = ''

def jprint(obj):
    text = json.dumps(obj, sort_keys=True, indent=2)
    print(text)

def getWallet(a, sb, nb, eb):
    p = {
        "module": 'account',
        "action": 'tokentx',
        "contractaddress": a,
        "startblock": sb,
        "endblock": nb,
        "sort": "asc",
        "apikey": API_KEY
    }

    buyList = []
    sellList = []

    while int(p["endblock"]) != eb:
        try:
            if 5000 < (eb - int(p["startblock"])):    
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 5000)
            elif 1000 < (eb - int(p["startblock"])):
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 1000)
            elif 500 < (eb - int(p["startblock"])):
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 500)
            elif 100 < (eb - int(p["startblock"])):
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 100)
            elif 10 < (eb - int(p["startblock"])):
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 10)
            else:
                response = requests.get('https://api.etherscan.io/api?', params=p)
                response.raise_for_status()
                tx = response.json()
                jprint(tx)
                for x in range(len(tx['result'])):
                    sym = tx['result'][x]['tokenName']
                    buy_address = tx['result'][x]['to']
                    buyList.append(buy_address)
                    sell_address = tx['result'][x]['from']
                    sellList.append(sell_address)

                p["startblock"] = str(int(p["endblock"]) + 1)
                p["endblock"] = str(int(p["endblock"]) + 1)
                if int(p["startblock"]) == int(p["endblock"]):
                    print("Final Block: ", p["endblock"])
                    break          
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')

    with open(sym + '_buy.txt', 'w') as f:
        for h in list(dict.fromkeys(buyList)):
            f.write(str(h)+'\n')

    with open(sym + '_sell.txt', 'w') as f:
        for h in list(dict.fromkeys(sellList)):
            f.write(str(h)+'\n')

    print("Buy wallet count: ", len(buyList))
    print("Sell wallet count: ", len(sellList))
