 json
{
  "annotations": [
    {
      "type": "lifetime",
      "name": "a"
    },
    {
      "type": "type",
      "name": "T",
      "traits": [
        "Show"
      ]
    }
  ],
  "arguments": [
    {
      "name": "x",
      "lifetime": "a",
      "type": "T"
    },
    {
      "name": "x",
      "lifetime": "a",
      "type": "T"
    }
  ],
  "return": {
    "type": "T",
    "lifetime": "a"
  }
}
