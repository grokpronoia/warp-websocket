import sys
from rawConversion import dateConverter
from blockConversion import blockNumber
from transactionAddress import getWallet

def main():
    start = dateConverter(sys.argv[1], "before")
    end = dateConverter(sys.argv[2], "after")
    first = int(blockNumber(start, "before"))
    next = first + 5000
    last = int(blockNumber(end, "after"))
    getWallet(sys.argv[3], first, next, last)

if __name__ == "__main__":
    main()
