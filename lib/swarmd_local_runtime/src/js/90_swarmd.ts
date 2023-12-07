import * as tls from "ext:deno_net/02_tls.js";
import * as net from "ext:deno_net/01_net.js";
import * as http from "ext:deno_http/01_http.js";
import * as serve from "ext:deno_http/00_serve.js";

const core = globalThis.Deno.core;
const ops = core.ops;

const unusedLocal = { tls, serve };

/**
 * To reproduce the behavior of Swarmd Edge.
 */
function fakeHttpSwarmdCreateConnection() {
  const conn = net.listen({ port: 13337 });
  return conn;
}

function fakeFetchRequest(conn) {
  const rid = ops.op_http_start(conn.rid);
  return new http.HttpConn(rid, "http://no-addr.swarmd.io", "http://local-no-addr.swarmd.io");
}

const swarmdNs = {
  fakeHttpSwarmdCreateConnection,
  fakeFetchRequest,
  upgradeWebSocket: http.upgradeWebSocket,
}

export { swarmdNs }
