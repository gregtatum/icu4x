window.BENCHMARK_DATA = {
  "lastUpdate": 1610471376300,
  "repoUrl": "https://github.com/gregtatum/icu4x",
  "entries": {
    "Heap – components/datetime": [
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
        "date": 1610402315245,
        "tool": "ndjson",
        "benches": [
          {
            "name": "memory – Total Heap Allocations",
            "value": 20258,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "memory – Heap at Global Memory Max",
            "value": 9360,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "memory – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log – Total Heap Allocations",
            "value": 21778,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log – Heap at Global Memory Max",
            "value": 9616,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      },
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
          "id": "d33026561021e8edff3df54839bcf83017cc76bd",
          "message": "Add a CI job to collect memory benchmarks\n\nThis uses a custom fork of the benchmarking tool to add support for\nndjson as a tool. This ndjson uses the same format as the internal json\nstructure of the benchmarking tool. This allows for fully customizing\nthe output of the data, and what information is collected.",
          "timestamp": "2021-01-12T10:50:26-06:00",
          "tree_id": "4898da1714ec141ecac2f48a89396319962902c9",
          "url": "https://github.com/gregtatum/icu4x/commit/d33026561021e8edff3df54839bcf83017cc76bd"
        },
        "date": 1610471371782,
        "tool": "ndjson",
        "benches": [
          {
            "name": "work_log – Total Heap Allocations",
            "value": 21778,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log – Heap at Global Memory Max",
            "value": 9616,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}