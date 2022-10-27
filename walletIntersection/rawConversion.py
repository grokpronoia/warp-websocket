import datetime

"""
def dateConverter(d):
    raw = str(d)
    con = datetime.datetime.strptime(raw, "%d/%m/%Y")
    utc = int(datetime.datetime.timestamp(con))
    return utc
"""

def dateConverter(d, ba):
    raw = str(d)
    if ba == "before":
        raws = raw + "T00:00:01+00:00"
    else:
        raws = raw + "T23:59:59+00:00"
    con = datetime.datetime.strptime(raws, "%Y-%m-%dT%H:%M:%S+00:00")
    gmt = int(datetime.datetime.timestamp(con))
    return gmt
