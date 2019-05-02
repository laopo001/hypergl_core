/**
 * ProjectName: hypergl-demo
 * FilePath: \src\bootstrap.js
 * Created Date: Tuesday, October 2nd 2018, 11:50:38 pm
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Thursday, May 2nd 2019, 8:54:32 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */


// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./hypergl_core")
  .catch(e => console.error("Error importing `index.js`:", e));
