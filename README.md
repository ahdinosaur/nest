# nest

nested data structures with two-way binding to filesystem storage.

---

```
nest::Store(schema)
```

```
config = {
  path: '/etc/peach',
  indexKey: 'config',
  schema: {
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
