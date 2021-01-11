window.BENCHMARK_DATA = {
  "lastUpdate": 1610402290694,
  "repoUrl": "https://github.com/gregtatum/icu4x",
  "entries": {
    "Heap – components/locid": [
      {
        "commit": {
          "author": {
            "email": "tatum.creative@gmail.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "tatum.creative@gmail.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "distinct": true,
          "id": "af3b3a15b5d0d8f5ae0d67868dfd9f7cc808c86f",
          "message": "Fix matrix command",
          "timestamp": "2021-01-11T15:54:59-06:00",
          "tree_id": "948d3dfa51482c39019dd3d29cdf3261f57c3921",
          "url": "https://github.com/gregtatum/icu4x/commit/af3b3a15b5d0d8f5ae0d67868dfd9f7cc808c86f"
        },
        "date": 1610402288623,
        "tool": "ndjson",
        "benches": [
          {
            "name": "syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 553,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 344,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids – Total Heap Allocations",
            "value": 1262,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids – Heap at Global Memory Max",
            "value": 741,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}