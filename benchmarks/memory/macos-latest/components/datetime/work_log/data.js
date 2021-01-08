window.BENCHMARK_DATA = {
  "lastUpdate": 1610142330887,
  "repoUrl": "https://github.com/gregtatum/icu4x",
  "entries": {
    "Heap – components/datetime work_log": [
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
          "id": "a23c3a8d7eade45f4dcd6981571b0fc8ce198310",
          "message": "Fix path 2",
          "timestamp": "2021-01-08T15:39:59-06:00",
          "tree_id": "8b4f40d1be67789829a31ba629ea0ceb4fc1308b",
          "url": "https://github.com/gregtatum/icu4x/commit/a23c3a8d7eade45f4dcd6981571b0fc8ce198310"
        },
        "date": 1610142330106,
        "tool": "ndjson",
        "benches": [
          {
            "name": "heap total allocations",
            "value": 21795,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "heap at global memory max",
            "value": 9620,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "heap at end of program execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}