# nest

nested data structures with two-way binding to filesystem storage.

```
nest::Database(config)
```

```
config = {
  path,
  structure
}
```

```
config = {
  path: '/etc/peach',
  indexKey: 'config',
  structure: {
    config: ['toml', 'hjson'],
    dyndns: {
      host: {
        origins: 'fs-list',
        guests: 'fs-map'
      },
      guest: {
        hosts: 'fs-map'
      }
    },
  }
}
```

given structure, return two-way binding nested config object

- leaves:
  - yaml
  - fs
  - json
  - toml
