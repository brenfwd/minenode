# minenode_napi

This crate contains the binding code for JS interop. This will eventually include the modding API, such as event listeners, interceptors, etc.

NAPI-RS is used to produce a NodeJS native module, which can be be loaded from NodeJS or Bun (preferred). See the `js` directory in the project root for an example.
