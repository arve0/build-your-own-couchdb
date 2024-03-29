# Build your own CouchDB
Hi! I'm Arve and this is my adventure into building my own modern CouchDB with Rust.

## What is CouchDB?
[CouchDB] is an key-value database written in Erlang, initially released in 2005. It has a simple replication protocol over HTTP, using revisions to ensure eventual consistency.

## Why I'm building my own?
CouchDB leaves me longing for better interoperability with modern browsers. Specifically I want "real time" replication to IndexedDB, which is unpleasant with regular CouchDB. The unpleasantness is mainly due to the revision mechanism, which is fairly Erlang specific. Revisions hashes are calculated using Erlang data structures and md5, both which are not native in browsers. Of course it is possible to achieve the revision calculation with some extra libraries. Still, I think it will be a fun challenge to implement a modern CouchDB variant after the *build your own x*-pattern.

With no further ado, let’s start the journey and look at the objectives.

## Objectives
Decisions made should reflect the main goals, they are:

### Goals
- Easy interoperability with other programming environments.
- Efficient and simple syncing.
- Use browser native protocols / APIs.
- Availability and Partition tolerance of [CAP].

To help finishing the project, some non-goals will restrict scope and complexity:

### Non goals
- Compatibility with CouchDB.
- Writing low level code.
- Exstensive server side logic, like index lookup and [design documents].
- Consistency of CAP.

[CouchDB]: https://couchdb.apache.org/
[CAP]: https://en.wikipedia.org/wiki/CAP_theorem
[design documents]: https://docs.couchdb.org/en/master/ddocs/index.html
