# Build your own CouchDB
Hi! I'm Arve and this is my adventure into building my own modern CouchDB with Rust.

## What is CouchDB?
[CouchDB] is an key-value database written in Erlang, initially released in 2005. It has a simple replication protocol over HTTP, using revisions to ensure eventual consistency.

## Why I'm building my own?
CouchDB leaves me longing for better interoperability with modern browsers. Specifically I want "real time" replication to IndexedDB, which is unpleasant with regular CouchDB. Mainly, this is due to the revision mechanism, which is fairly Erlang specific. Revisions hashes are calculated using Erlang data structures and md5, both which are not native in browsers. Of course possible to achieve the revision calculation with some extra libraries. Still, I think it will be a fun challenge to implement a modern CouchDB variant after the *build your own x*-pattern.

With no further ado, let’s start the journey and look at the objectives.

## Objectives
Decisions made when when building my own CouchDB should reflect the main goals, they are:

### Goals
- Easy interoperability with other programming environments.
- Efficient syncing.
- Use modern protocols.
- Availability and Partition tolerance of [CAP].

### Non goals
- Compatibility with CouchDB.
- Writing low level code.
- Server side lookup.
- Consistency of CAP.

[CouchDB]: https://couchdb.apache.org/
[CAP]: https://en.wikipedia.org/wiki/CAP_theorem
