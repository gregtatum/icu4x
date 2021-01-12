window.BENCHMARK_DATA = {
  "lastUpdate": 1610468927657,
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
          "id": "6a0e5f54056d07bb94cee6157baeaea52841f0bf",
          "message": "Prototype out action",
          "timestamp": "2021-01-05T13:36:29-06:00",
          "tree_id": "271e27b7e8b99ec65175d1162cb9924d198615c2",
          "url": "https://github.com/gregtatum/icu4x/commit/6a0e5f54056d07bb94cee6157baeaea52841f0bf"
        },
        "date": 1609876303326,
        "tool": "cargo",
        "benches": [
          {
            "name": "langid/overview",
            "value": 4035,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "locale/overview",
            "value": 8626,
            "range": "± 151",
            "unit": "ns/iter"
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
          "id": "4d6bb27c12b1b270ec9dd0890ad1dc02a0c278e9",
          "message": "Add a CI job to collect memory benchmarks\n\nThis uses a custom fork of the benchmarking tool to add support for\nndjson as a tool. This ndjson uses the same format as the internal json\nstructure of the benchmarking tool. This allows for fully customizing\nthe output of the data, and what information is collected.",
          "timestamp": "2021-01-12T09:24:12-06:00",
          "tree_id": "80bacb8b1b39aa62e3ab4a09db5c9ed6a6ea72bc",
          "url": "https://github.com/gregtatum/icu4x/commit/4d6bb27c12b1b270ec9dd0890ad1dc02a0c278e9"
        },
        "date": 1610466246426,
        "tool": "cargo",
        "benches": [
          {
            "name": "langid/overview",
            "value": 3832,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "locale/overview",
            "value": 7789,
            "range": "± 320",
            "unit": "ns/iter"
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
          "id": "503d9579e5136bf75746113a1d222253cc4c6fe6",
          "message": "Add a CI job to collect memory benchmarks\n\nThis uses a custom fork of the benchmarking tool to add support for\nndjson as a tool. This ndjson uses the same format as the internal json\nstructure of the benchmarking tool. This allows for fully customizing\nthe output of the data, and what information is collected.",
          "timestamp": "2021-01-12T10:10:32-06:00",
          "tree_id": "a2e75e0337192bcb1986e426a77c852fa41cca15",
          "url": "https://github.com/gregtatum/icu4x/commit/503d9579e5136bf75746113a1d222253cc4c6fe6"
        },
        "date": 1610468926648,
        "tool": "cargo",
        "benches": [
          {
            "name": "langid/overview",
            "value": 4122,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "locale/overview",
            "value": 9578,
            "range": "± 368",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}