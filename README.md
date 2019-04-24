# nest

nested data structures using the filesystem as your database.

_work in progress_

## api sketch

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
