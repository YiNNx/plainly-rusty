# plainly-rusty

It originally started as a simple, almost mundane backend rewrite project, for my old blog [backend](https://github.com/YiNNx/Blog-BE). However, as I delved into learning Rust and explored Rust crates associated with web development, it turned to an exploratory venture, to explore the potential of replacing conventional CRUD repetitive tasks through GraphQL & higher-level code abstraction with the features of Rust & Seaography in backend projects.

## Features

- The whole project is based on the [Seaography](https://www.sea-ql.org/Seaography/) and the code framework generated from its cli, with dynamic-schema [async-graphql](https://github.com/async-graphql/async-graphql) as the GraghQL library, Actix as the web framework, and SeaORM & Postgres as the ORM & database, providing rich and flexible GraphQL query (nested relation, pagination, filtering, ordering, etc.)
- Implement permission control through a custom guard system and a custom resolver registration system. 
- Use GitHub OAuth2 for authentication, and avoid the burden of building any user system while designing data schema, just using stateless JWT.

## How to Run

1. Set the postgres and import the schema with `scripts/plainly_rusty_create.sql`

   ```shell
   psql -q postgres://{user}:{pwd}@{addr}/plainlyrusty < scripts/plainly-rusty-schema.sql
   ```

2. Modify or replace the default configuration file, `configs/default.toml`.

3. Run with env variables:

   ```shell
   RUST_LOG={log_level} CONFIG_FILE={file_path} cargo run
   ```


## Query Sample

```text
query Posts {
  posts(
    filters: { status: { eq: PUBLIC } }
    orderBy: { time: ASC }
    pagination: { offset: { limit: 10, offset: 0 } }
  ) {
    nodes {
      id
      title
      time
      content
      summary
      comments(
        orderBy: { time: ASC }
        pagination: { offset: { limit: 2, offset: 0 } }
      ) {
        nodes {
          parentId
          githubId
          time
          content
        }
      }
      tags(orderBy: { name: ASC }) {
        nodes {
          name
        }
      }
    }
  }
}

```

On the GraphQL Playground:

![screenshot](https://cdn.just-plain.fun/img/image-20231123130247544.png)
