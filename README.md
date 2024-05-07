## Foodchain - local OpenFoodFacts Lookup API

Requires that you download the OpenFoodFadcts mongodb (55GB uncompressed):
https://world.openfoodfacts.org/data

Restore that to your mongo instance and then point your config at it.

You can then lookup product information by UPC:

http://localhost:8080/upc?code=0026400232209

*Server Response:*

```
{
  "objects": [
    {
      "code": "0026400232209",
      "url": null,
      "quantity": "1.89 liter",
      "product_name": "40% Whipping Cream",
      "product_quantity": 1890.0000000000002,
      "generic_name": null,
      "brands": "Darigold",
      "nutriments": {
        "energy_value": 400,
        "energy_unit": "kcal",
        "fat_100g": 40
      }
    }
  ]
}
```