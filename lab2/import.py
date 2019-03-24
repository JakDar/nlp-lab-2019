from elasticsearch import Elasticsearch
from elasticsearch import helpers
import os
es = Elasticsearch()

actions = [{
    "_index": "law_index4",
    "_id": i,
    "_source": {
        "content": open("ustawy/" + file).read(),
        "filename": file
    }
} for i, file in enumerate(os.listdir("ustawy"))]

helpers.bulk(es, actions)
