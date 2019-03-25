from elasticsearch import Elasticsearch
from elasticsearch import helpers
import os
es = Elasticsearch(retry_on_timeout=True)

actions = [{
    "_index": "law_index6",
    "_id": i,
    "_type": "_doc",
    "_source": {
        "content": open("../ustawy/" + file).read(),
        "filename": file
    }
} for i, file in enumerate(os.listdir("../ustawy"))]

helpers.bulk(es, actions)
