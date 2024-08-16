import("../pkg/index.js").catch(console.error);
// import { ws_ping } from "../pkg/index.js";

// I think the import above will resolve ws_ping import
// import { ws_ping } from '../pkg/index.js'


async function run() {
  // const ws_ping = import("../pkg/index.js");
  // const _host = 'ws://127.0.0.1:4011/socket';
  const endpoint = '/todo_endpoint';
  const msg = "Sup, dude";


  // Here it calls main_js but is not calling ws_ping
  // TODO: resolve ws_ping import
  //ws_ping(endpoint, msg);
}

run();


