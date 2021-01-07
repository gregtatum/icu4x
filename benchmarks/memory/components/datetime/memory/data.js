window.BENCHMARK_DATA = {
  "lastUpdate": 1610056835606,
  "repoUrl": "https://github.com/gregtatum/icu4x",
  "entries": {
    "Rust Benchmark": [
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
          "id": "918d2cff72188183790c9fb8d70822600c25da67",
          "message": "Update path for the benchmark data",
          "timestamp": "2021-01-07T15:46:51-06:00",
          "tree_id": "f871e27a92532aeea688d05e5243294f8307a129",
          "url": "https://github.com/gregtatum/icu4x/commit/918d2cff72188183790c9fb8d70822600c25da67"
        },
        "date": 1610056164992,
        "tool": "ndjson",
        "benches": [
          {
            "name": "example components/datetime/memory",
            "value": 20281,
            "unit": "bytes",
            "biggerIsBetter": false,
            "range": "± 0",
            "extra": "heap total"
          },
          {
            "name": "example components/datetime/memory",
            "value": 9360,
            "unit": "bytes",
            "biggerIsBetter": false,
            "range": "± 0",
            "extra": "heap at global max"
          },
          {
            "name": "example components/datetime/memory",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false,
            "range": "± 0",
            "extra": "heap at end"
          }
        ]
      }
    ],
    "Heap – components/datetime memory": [
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
          "id": "13146f61e0e99176adba872a78f24ddcd77cdc92",
          "message": "Fine-tune labeling",
          "timestamp": "2021-01-07T15:58:13-06:00",
          "tree_id": "ecc5d8dc60310adbb27af0e31abd541607c14e8e",
          "url": "https://github.com/gregtatum/icu4x/commit/13146f61e0e99176adba872a78f24ddcd77cdc92"
        },
        "date": 1610056833799,
        "tool": "ndjson",
        "benches": [
          {
            "name": "heap total allocations",
            "value": 20281,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "heap at global memory max",
            "value": 9360,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "heap at end of program execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}