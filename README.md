## Foodchain - local OpenFoodFacts Lookup API

Requires that you download the OpenFoodFadcts mongodb (55GB uncompressed):
https://world.openfoodfacts.org/data

Restore that to your mongo instance and then point your config at it.

You can then lookup product information by UPC:

http://localhost:8080/upc?code=0026400232209
